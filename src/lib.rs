//! Supported client contracts. No Case semantics, persistence or server implementation.
pub mod conformance;
mod contracts;
mod local;
pub mod projections;
pub mod wire;
pub use contracts::*;
pub use local::{observe, restart, start, stop, HostClient};
/// OS transport identity/security shared with a conforming local server.
pub mod platform {
    pub use crate::local::{
        canonical_home, home_identity, peer_credentials, read_discovery, validate_discovery,
        verify_peer_same_user, PeerCredentials,
    };
}
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Failure of communication, separate from a typed application refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientError {
    pub code: String,
    /// True means that observing the operation identity is required before retry.
    pub outcome_indeterminate: bool,
}
impl ClientError {
    pub fn after_dispatch(code: String) -> Self {
        let outcome_indeterminate = true;
        Self {
            code,
            outcome_indeterminate,
        }
    }
}
impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code.fmt(f)
    }
}
impl std::error::Error for ClientError {}

/// A transport carries a public request; it cannot implement Case semantics.
pub trait ClientTransport {
    fn execute(&self, request: OperationRequest) -> Result<OperationResult, ClientError>;
}
pub struct LocalTransport {
    pub home: std::path::PathBuf,
    pub client_kind: ClientKind,
}
impl ClientTransport for LocalTransport {
    fn execute(&self, request: OperationRequest) -> Result<OperationResult, ClientError> {
        HostClient::connect(&self.home, self.client_kind.clone())
            .map_err(|code| ClientError {
                code,
                outcome_indeterminate: false,
            })?
            .call_typed(request)
    }
}
