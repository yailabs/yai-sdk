//! Typed client orchestration. Core still owns admission and all semantic effects.
use crate::{projections::ApplicationCatalog, ClientError, ClientTransport,
    OperationError, OperationRequest, OperationResult, ResultState, APPLICATION_PROTOCOL,
    };
use serde::{de::DeserializeOwned, Serialize};

/// A semantic response remains observable even when it is not successful.
#[derive(Clone, Debug)]
pub struct Response<T> {
    /// Exact received envelope, including diagnostic data on non-success states.
    pub raw: OperationResult,
    pub operation_ref: String,
    pub correlation_ref: String,
    pub state: ResultState,
    pub data: Option<T>,
    pub error: Option<OperationError>,
}

/// Client failures never replace a Core semantic refusal with a transport label.
#[derive(Debug)]
pub enum Error {
    Transport(ClientError),
    /// This connected catalog does not advertise the requested operation.
    UnsupportedOperation(String),
    ContractMismatch(String),
    InvalidRequest(String),
    /// A response was received but cannot be interpreted. Never blindly replay.
    InvalidResponse { reason: String, response: OperationResult },
    DiscoveryRefused(OperationResult),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport(error) => error.fmt(f),
            Self::UnsupportedOperation(id) => write!(f, "unsupported_operation:{id}"),
            Self::ContractMismatch(id) => write!(f, "operation_contract_mismatch:{id}"),
            Self::InvalidRequest(reason) => write!(f, "invalid_request:{reason}"),
            Self::InvalidResponse { reason, .. } => write!(f, "invalid_response:{reason}"),
            Self::DiscoveryRefused(result) => write!(f, "discovery_refused:{:?}", result.result_state),
        }
    }
}
impl std::error::Error for Error {}

/// Typed operation descriptors bind request and response representations.
/// Custom operations remain possible through the explicit low-level transport.
pub trait Operation: Serialize {
    type Output: DeserializeOwned;
    const ID: &'static str;
    const INPUT_CONTRACT: Option<&'static str> = None;
    const OUTPUT_CONTRACT: Option<&'static str> = None;
}

/// A catalog is an observation, never an authority grant. Every call still passes
/// through Core's current admission. Reconnect/replacement requires rediscovery.
pub struct Client<T> {
    transport: T,
    catalog: ApplicationCatalog,
}
impl<T: ClientTransport> Client<T> {
    pub fn discover(transport: T, correlation: &str) -> Result<Self, Error> {
        validate_correlation(correlation)?;
        let response = transport.execute(OperationRequest {
            protocol: APPLICATION_PROTOCOL.into(),
            operation_ref: "application.capabilities".into(),
            correlation_ref: correlation.into(),
            input: serde_json::json!({}),
        }).map_err(Error::Transport)?;
        validate_identity(&response, "application.capabilities", correlation)?;
        if response.result_state != ResultState::Success {
            return Err(Error::DiscoveryRefused(response));
        }
        let catalog = crate::validate_catalog_result(&response).map_err(|reason| Error::InvalidResponse {
            reason, response: response.clone(),
        })?;
        Ok(Self { transport, catalog })
    }

    pub fn capabilities(&self) -> &ApplicationCatalog { &self.catalog }

    pub fn supports(&self, operation: &str) -> bool {
        self.catalog.operations.iter().any(|op| op.operation_id == operation)
    }

    /// Correlation is not an idempotency key. Effectful request types retain
    /// their own durable submission identity; this method never retries.
    pub fn execute<O: Operation>(&self, correlation: &str, input: &O) -> Result<Response<O::Output>, Error> {
        validate_correlation(correlation)?;
        let descriptor = self.catalog.operations.iter().find(|op| op.operation_id == O::ID)
            .ok_or_else(|| Error::UnsupportedOperation(O::ID.into()))?;
        if O::INPUT_CONTRACT.is_some_and(|id| id != descriptor.input_contract)
            || O::OUTPUT_CONTRACT.is_some_and(|id| id != descriptor.output_contract)
        {
            return Err(Error::ContractMismatch(O::ID.into()));
        }
        let input = serde_json::to_value(input).map_err(|error| Error::InvalidRequest(error.to_string()))?;
        let response = self.transport.execute(OperationRequest {
            protocol: APPLICATION_PROTOCOL.into(), operation_ref: O::ID.into(),
            correlation_ref: correlation.into(), input,
        }).map_err(Error::Transport)?;
        validate_identity(&response, O::ID, correlation)?;
        let data = match response.result_state {
            ResultState::Success => Some(decode(&response)?),
            // A partial response may carry a documented projection; never turn
            // it into Success or require a data body for a refusal.
            ResultState::Partial if response.data.is_some() => Some(decode(&response)?),
            _ => None,
        };
        if response.error.as_ref().is_some_and(|error| error.result_state != response.result_state)
            || (response.result_state == ResultState::Success && response.error.is_some())
        {
            return Err(Error::InvalidResponse { reason: "contradictory_result_state".into(), response });
        }
        Ok(Response { raw: response.clone(), operation_ref: response.operation_ref, correlation_ref: response.correlation_ref,
            state: response.result_state, data, error: response.error })
    }
}

/// Local transport pinned to one authenticated Host instance. Discovery from a
/// retired instance cannot silently authorize routing against its replacement.
pub struct BoundLocalTransport {
    home: std::path::PathBuf,
    kind: crate::ClientKind,
    instance_id: String,
}
impl BoundLocalTransport {
    pub fn connect(home: impl AsRef<std::path::Path>, kind: crate::ClientKind) -> Result<Self, ClientError> {
        let client = crate::HostClient::connect(home.as_ref(), kind.clone()).map_err(|code| ClientError {
            code, outcome_indeterminate: false,
        })?;
        Ok(Self { home: home.as_ref().to_path_buf(), kind,
            instance_id: client.discovery.instance_id.clone() })
    }
    pub fn instance_id(&self) -> &str { &self.instance_id }
}
impl ClientTransport for BoundLocalTransport {
    fn execute(&self, request: OperationRequest) -> Result<OperationResult, ClientError> {
        let client = crate::HostClient::connect(&self.home, self.kind.clone()).map_err(|code| ClientError {
            code, outcome_indeterminate: false,
        })?;
        if client.discovery.instance_id != self.instance_id {
            return Err(ClientError { code: "host_replaced_rediscovery_required".into(), outcome_indeterminate: false });
        }
        client.call_typed(request)
    }
}

fn validate_correlation(value: &str) -> Result<(), Error> {
    if value.trim().is_empty() { Err(Error::InvalidRequest("empty_correlation".into())) }
    else { Ok(()) }
}
fn validate_identity(response: &OperationResult, operation: &str, correlation: &str) -> Result<(), Error> {
    if response.operation_ref != operation || response.correlation_ref != correlation {
        Err(Error::InvalidResponse { reason: "response_identity_mismatch".into(), response: response.clone() })
    } else { Ok(()) }
}
fn decode<T: DeserializeOwned>(response: &OperationResult) -> Result<T, Error> {
    let data = response.data.clone().ok_or_else(|| Error::InvalidResponse {
        reason: "missing_data".into(), response: response.clone(),
    })?;
    serde_json::from_value(data).map_err(|error| Error::InvalidResponse {
        reason: format!("projection_mismatch:{error}"), response: response.clone(),
    })
}
