//! YVEX-owned operational facts projected through the public management v1
//! contract. This crate has no dependency on the YAI semantic client.
//!
//! The current remote contract is deliberately read-only: device identity and
//! host status. Model/runtime mutation is not represented by this crate.

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

pub const REQUEST_SCHEMA: &str = "yvex.management.request.v1";
pub const RESPONSE_SCHEMA: &str = "yvex.management.response.v1";
/// Exact public management operations represented by this SDK projection.
pub const MANAGEMENT_OPERATIONS: [&str; 2] = ["device.describe", "host.status"];
pub const MAX_RESPONSE_BYTES: usize = 8192;

#[derive(Clone, Debug)]
pub struct SshConnection {
    /// Independently approved OpenSSH known-hosts file. Never populated from
    /// the peer's unauthenticated JSON response.
    pub pinned_known_hosts: PathBuf,
    pub enrolled_client_key: PathBuf,
    pub address: String,
    pub port: u16,
    pub user: String,
    pub expected_device_identity: String,
    pub expected_peer_identity: String,
}

impl SshConnection {
    pub fn validate(&self) -> Result<(), ClientError> {
        if self.port == 0
            || !valid_user(&self.user)
            || !valid_address(&self.address)
            || !valid_identity(&self.expected_device_identity)
            || !valid_identity(&self.expected_peer_identity)
            || !self.pinned_known_hosts.is_absolute()
            || !self.enrolled_client_key.is_absolute()
        {
            return Err(ClientError::InvalidConfiguration);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientError {
    InvalidConfiguration,
    InvalidResponse,
    ResponseTooLarge,
    CorrelationMismatch,
    IdentityMismatch,
    TransportUnavailable,
    Timeout,
    Refused(String),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Refused(reason) => write!(formatter, "YVEX refused: {reason}"),
            other => write!(formatter, "YVEX management client: {other:?}"),
        }
    }
}

impl std::error::Error for ClientError {}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HostState {
    Running,
    Stopped,
    Unavailable,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct DeviceDescription {
    pub device_identity: String,
    pub authenticated_peer: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct HostStatus {
    pub device_identity: String,
    pub authenticated_peer: String,
    pub state: HostState,
    pub engine_count: Option<u64>,
    pub loaded_engine_count: Option<u64>,
    pub session_count: Option<u64>,
}

#[derive(Serialize)]
struct Request<'a> {
    schema: &'static str,
    request_id: &'a str,
    operation: &'a str,
}

#[derive(Deserialize)]
struct Response {
    schema: String,
    request_id: Option<String>,
    status: String,
    device_identity: Option<String>,
    authenticated_peer: Option<String>,
    host_state: Option<HostState>,
    engine_count: Option<u64>,
    loaded_engine_count: Option<u64>,
    session_count: Option<u64>,
    reason: Option<String>,
}

pub struct ManagementClient {
    connection: SshConnection,
    timeout: Duration,
}

impl ManagementClient {
    pub fn new(connection: SshConnection) -> Result<Self, ClientError> {
        connection.validate()?;
        Ok(Self {
            connection,
            timeout: Duration::from_secs(15),
        })
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Result<Self, ClientError> {
        if timeout.is_zero() || timeout > Duration::from_secs(120) {
            return Err(ClientError::InvalidConfiguration);
        }
        self.timeout = timeout;
        Ok(self)
    }

    pub fn device_describe(&self) -> Result<DeviceDescription, ClientError> {
        let response = self.invoke(MANAGEMENT_OPERATIONS[0])?;
        Ok(DeviceDescription {
            device_identity: response.device_identity.unwrap(),
            authenticated_peer: response.authenticated_peer.unwrap(),
        })
    }

    pub fn host_status(&self) -> Result<HostStatus, ClientError> {
        let response = self.invoke(MANAGEMENT_OPERATIONS[1])?;
        let state = response.host_state.ok_or(ClientError::InvalidResponse)?;
        if state == HostState::Running {
            if response.engine_count.is_none()
                || response.loaded_engine_count.is_none()
                || response.session_count.is_none()
            {
                return Err(ClientError::InvalidResponse);
            }
        } else if response.engine_count.is_some()
            || response.loaded_engine_count.is_some()
            || response.session_count.is_some()
        {
            return Err(ClientError::InvalidResponse);
        }
        Ok(HostStatus {
            device_identity: response.device_identity.unwrap(),
            authenticated_peer: response.authenticated_peer.unwrap(),
            state,
            engine_count: response.engine_count,
            loaded_engine_count: response.loaded_engine_count,
            session_count: response.session_count,
        })
    }

    fn invoke(&self, operation: &str) -> Result<Response, ClientError> {
        let mut random = [0_u8; 32];
        getrandom::fill(&mut random).map_err(|_| ClientError::TransportUnavailable)?;
        let request_id: String = random.iter().map(|byte| format!("{byte:02x}")).collect();
        let mut request = serde_json::to_vec(&Request {
            schema: REQUEST_SCHEMA,
            request_id: &request_id,
            operation,
        })
        .map_err(|_| ClientError::InvalidResponse)?;
        request.push(b'\n');

        // OpenSSH owns host-key verification. No remote command is passed: the
        // enrolled key selects YVEX's forced management command on the server.
        let mut child = Command::new("ssh")
            .arg("-T")
            .arg("-F")
            .arg("/dev/null")
            .arg("-o")
            .arg("BatchMode=yes")
            .arg("-o")
            .arg("PreferredAuthentications=publickey")
            .arg("-o")
            .arg("PasswordAuthentication=no")
            .arg("-o")
            .arg("KbdInteractiveAuthentication=no")
            .arg("-o")
            .arg("GSSAPIAuthentication=no")
            .arg("-o")
            .arg("ClearAllForwardings=yes")
            .arg("-o")
            .arg("ForwardAgent=no")
            .arg("-o")
            .arg("StrictHostKeyChecking=yes")
            .arg("-o")
            .arg("IdentitiesOnly=yes")
            .arg("-o")
            .arg("ConnectionAttempts=1")
            .arg("-o")
            .arg("ConnectTimeout=5")
            .arg("-o")
            .arg(format!(
                "UserKnownHostsFile={}",
                self.connection.pinned_known_hosts.display()
            ))
            .arg("-i")
            .arg(&self.connection.enrolled_client_key)
            .arg("-p")
            .arg(self.connection.port.to_string())
            .arg(format!(
                "{}@{}",
                self.connection.user, self.connection.address
            ))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| ClientError::TransportUnavailable)?;
        if child.stdin.take().unwrap().write_all(&request).is_err() {
            let _ = child.kill();
            let _ = child.wait();
            return Err(ClientError::TransportUnavailable);
        }
        let status = match child.wait_timeout(self.timeout) {
            Ok(Some(status)) => status,
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(ClientError::Timeout);
            }
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(ClientError::TransportUnavailable);
            }
        };
        if !status.success() {
            return Err(ClientError::TransportUnavailable);
        }
        let mut output = Vec::new();
        child
            .stdout
            .take()
            .unwrap()
            .take((MAX_RESPONSE_BYTES + 1) as u64)
            .read_to_end(&mut output)
            .map_err(|_| ClientError::TransportUnavailable)?;
        if output.len() > MAX_RESPONSE_BYTES {
            return Err(ClientError::ResponseTooLarge);
        }
        parse_response(&output, &request_id, &self.connection)
    }
}

fn parse_response(
    bytes: &[u8],
    request_id: &str,
    connection: &SshConnection,
) -> Result<Response, ClientError> {
    if bytes.last() != Some(&b'\n') || bytes[..bytes.len() - 1].contains(&b'\n') {
        return Err(ClientError::InvalidResponse);
    }
    let response: Response =
        serde_json::from_slice(bytes).map_err(|_| ClientError::InvalidResponse)?;
    if response.schema != RESPONSE_SCHEMA {
        return Err(ClientError::InvalidResponse);
    }
    if response.status == "refused" {
        let reason = response.reason.ok_or(ClientError::InvalidResponse)?;
        match response.request_id.as_deref() {
            Some(received) if received != request_id => {
                return Err(ClientError::CorrelationMismatch)
            }
            None if !matches!(
                reason.as_str(),
                "malformed_request" | "peer_revoked_or_authority_unavailable"
            ) =>
            {
                return Err(ClientError::InvalidResponse);
            }
            _ => {}
        }
        return Err(ClientError::Refused(reason));
    }
    if response.status != "ok" || response.reason.is_some() {
        return Err(ClientError::InvalidResponse);
    }
    if response.request_id.as_deref() != Some(request_id) {
        return Err(ClientError::CorrelationMismatch);
    }
    if response.device_identity.as_deref() != Some(&connection.expected_device_identity)
        || response.authenticated_peer.as_deref() != Some(&connection.expected_peer_identity)
    {
        return Err(ClientError::IdentityMismatch);
    }
    Ok(response)
}

fn valid_identity(identity: &str) -> bool {
    identity
        .strip_prefix("ssh-ed25519:sha256:")
        .is_some_and(|hex| {
            hex.len() == 64
                && hex
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        })
}

fn valid_user(user: &str) -> bool {
    !user.is_empty()
        && user
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && user
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
}

fn valid_address(address: &str) -> bool {
    !address.is_empty()
        && !address.starts_with('-')
        && address
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b".-_:".contains(&byte))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn connection() -> SshConnection {
        SshConnection {
            pinned_known_hosts: "/tmp/known_hosts".into(),
            enrolled_client_key: "/tmp/client_key".into(),
            address: "example.test".into(),
            port: 2222,
            user: "yvex".into(),
            expected_device_identity: format!("ssh-ed25519:sha256:{}", "a".repeat(64)),
            expected_peer_identity: format!("ssh-ed25519:sha256:{}", "b".repeat(64)),
        }
    }

    #[test]
    fn validates_exact_configuration() {
        assert!(connection().validate().is_ok());
        let mut changed = connection();
        changed.expected_device_identity = "not-a-key".into();
        assert_eq!(changed.validate(), Err(ClientError::InvalidConfiguration));
        changed = connection();
        changed.address = "-oProxyCommand=evil".into();
        assert_eq!(changed.validate(), Err(ClientError::InvalidConfiguration));
    }

    #[test]
    fn exact_identity_and_correlation_are_required() {
        let config = connection();
        let payload = format!("{{\"schema\":\"{RESPONSE_SCHEMA}\",\"request_id\":\"{}\",\"status\":\"ok\",\"device_identity\":\"{}\",\"authenticated_peer\":\"{}\"}}\n", "c".repeat(64), config.expected_device_identity, config.expected_peer_identity);
        assert!(parse_response(payload.as_bytes(), &"c".repeat(64), &config).is_ok());
        assert!(matches!(
            parse_response(payload.as_bytes(), &"d".repeat(64), &config),
            Err(ClientError::CorrelationMismatch)
        ));
        let mut wrong = config.clone();
        wrong.expected_device_identity = format!("ssh-ed25519:sha256:{}", "d".repeat(64));
        assert!(matches!(
            parse_response(payload.as_bytes(), &"c".repeat(64), &wrong),
            Err(ClientError::IdentityMismatch)
        ));
    }

    #[test]
    fn malformed_and_refused_responses_do_not_become_facts() {
        let config = connection();
        assert!(matches!(
            parse_response(b"{}\n{}\n", "id", &config),
            Err(ClientError::InvalidResponse)
        ));
        let refused = format!("{{\"schema\":\"{RESPONSE_SCHEMA}\",\"request_id\":null,\"status\":\"refused\",\"reason\":\"peer_revoked_or_authority_unavailable\"}}\n");
        assert_eq!(
            parse_response(refused.as_bytes(), "id", &config).err(),
            Some(ClientError::Refused(
                "peer_revoked_or_authority_unavailable".into()
            ))
        );
        let foreign = format!("{{\"schema\":\"{RESPONSE_SCHEMA}\",\"request_id\":\"foreign\",\"status\":\"refused\",\"reason\":\"unsupported_operation\"}}\n");
        assert_eq!(
            parse_response(foreign.as_bytes(), "id", &config).err(),
            Some(ClientError::CorrelationMismatch)
        );
    }
}
