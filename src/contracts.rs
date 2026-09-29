use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use crate::projections::ApplicationCatalog;
use std::fs;
pub const HOST_PROTOCOL: &str = "yai.client.v1";
pub const HOST_DISCOVERY_SCHEMA: &str = "yai.local_host.discovery.v1";
pub const HOST_TELEMETRY_SCHEMA: &str = "yai.local_host.telemetry.v1";
pub const APPLICATION_PROTOCOL: &str = "yai.studio.application.v1";
pub const CAPABILITY_CATALOG_SCHEMA: &str = "yai.application_capability_catalog.v1";
const PROCESS_IDENTITY_SCHEMA: &str = "yai.local_process_identity.v1";
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OperationRequest {
    pub protocol: String,
    pub operation_ref: String,
    pub correlation_ref: String,
    #[serde(default)]
    pub input: Value,
}

pub use crate::workflows::{OperationError, OperationResult, ResultState, CaseUpdate};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RuntimeObservation {
    pub instance_id: String,
    pub pid: u32,
    pub process_identity: String,
    pub lifecycle: String,
    pub observed_at_unix_ms: u64,
    pub heartbeat_at_unix_ms: u64,
    pub worker_capacity: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_workers: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_workers: Option<usize>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientKind {
    External,
    Studio,
    Cli,
    Qualification,
}

impl ClientKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::External => "external",
            Self::Studio => "studio",
            Self::Cli => "cli",
            Self::Qualification => "qualification",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HostDiscovery {
    pub schema: String,
    pub protocol: String,
    pub endpoint: String,
    pub pid: u32,
    pub process_identity: LocalProcessIdentity,
    pub instance_id: String,
    pub started_at_unix_ms: u64,
    pub version: String,
    pub build: String,
    pub yai_home: String,
    pub yai_home_identity: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientAttachment {
    pub client_id: String,
    pub client_kind: ClientKind,
    pub pid: u32,
    pub connected_at_unix_ms: u64,
    pub last_seen_unix_ms: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HostTelemetry {
    pub schema: String,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_identity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at_unix_ms: Option<u64>,
    pub uptime_ms: u64,
    pub protocol: String,
    pub version: String,
    pub build: String,
    /// Client-side observation of the exact Host process executable link.
    /// `linked` does not prove that the process matches the latest source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executable_posture: Option<String>,
    /// Client-side comparison with the exact installed Core selected by this
    /// client. Inode equality is process identity, not product compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installed_executable_posture: Option<String>,
    pub yai_home: String,
    pub yai_home_identity: String,
    pub transport: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    pub endpoint_posture: String,
    pub application_readiness: String,
    pub connected_clients: usize,
    pub client_kinds: BTreeMap<String, usize>,
    pub clients: Vec<ClientAttachment>,
    pub event_sequence: u64,
    pub last_activity_unix_ms: u64,
    pub runtime_supervision: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_observation: Option<RuntimeObservation>,
}
#[derive(Clone, Debug)]
pub enum HostEvent {
    Case(CaseUpdate),
    Heartbeat(HostTelemetry),
    Shutdown(String),
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalProcessIdentity {
    pub schema: String,
    pub pid: u32,
    pub boot_id: String,
    pub start_ticks: u64,
}

impl LocalProcessIdentity {
    pub fn capture(pid: u32) -> Result<Self, String> {
        if pid <= 1 {
            return Err("process_identity_pid_forbidden".to_string());
        }
        #[cfg(target_os = "linux")]
        {
            let stat = fs::read_to_string(format!("/proc/{pid}/stat"))
                .map_err(|error| format!("process_identity_stat_unavailable: {error}"))?;
            let close = stat
                .rfind(')')
                .ok_or_else(|| "process_identity_stat_malformed".to_string())?;
            // Fields after comm begin at field 3; starttime is field 22 and
            // therefore index 19 in this suffix.
            let fields = stat[close + 1..].split_whitespace().collect::<Vec<_>>();
            let start_ticks = fields
                .get(19)
                .ok_or_else(|| "process_identity_starttime_missing".to_string())?
                .parse::<u64>()
                .map_err(|error| format!("process_identity_starttime_invalid: {error}"))?;
            let boot_id = fs::read_to_string("/proc/sys/kernel/random/boot_id")
                .map_err(|error| format!("process_identity_boot_id_unavailable: {error}"))?
                .trim()
                .to_string();
            let value = Self {
                schema: PROCESS_IDENTITY_SCHEMA.to_string(),
                pid,
                boot_id,
                start_ticks,
            };
            value.validate()?;
            Ok(value)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = pid;
            Err("process_identity_unsupported_platform".to_string())
        }
    }

    pub fn canonical_identity(&self) -> String {
        format!(
            "linux-process-v1:{}:{}:{}",
            self.boot_id, self.pid, self.start_ticks
        )
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != PROCESS_IDENTITY_SCHEMA
            || self.pid <= 1
            || self.boot_id.trim().is_empty()
            || self.start_ticks == 0
        {
            return Err("invalid_local_process_identity".to_string());
        }
        Ok(())
    }

    pub fn is_live(&self) -> bool {
        Self::capture(self.pid).is_ok_and(|current| current == *self)
    }
}

/// Independent protocol and product identities, never Git equality.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CompatibilityIdentity {
    pub core_version: String,
    pub protocol: String,
    pub application_protocol: String,
    pub capability_schema: String,
}
impl CompatibilityIdentity {
    pub fn current(core_version: &str) -> Self {
        Self {
            core_version: core_version.into(),
            protocol: HOST_PROTOCOL.into(),
            application_protocol: APPLICATION_PROTOCOL.into(),
            capability_schema: CAPABILITY_CATALOG_SCHEMA.into(),
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if semver::Version::parse(&self.core_version).is_err() {
            return Err("host_product_version_invalid".into());
        }
        if self.protocol != HOST_PROTOCOL
            || self.application_protocol != APPLICATION_PROTOCOL
            || self.capability_schema != CAPABILITY_CATALOG_SCHEMA
        {
            return Err("host_compatibility_mismatch".into());
        }
        Ok(())
    }
}

pub fn validate_catalog_result(result: &OperationResult) -> Result<ApplicationCatalog, String> {
    if result.operation_ref != "application.capabilities"
        || result.result_state != ResultState::Success
        || result.error.is_some()
    {
        return Err("catalog_result_not_success".into());
    }
    let catalog: ApplicationCatalog =
        serde_json::from_value(result.data.clone().ok_or("catalog_missing")?)
            .map_err(|e| format!("catalog_projection_invalid:{e}"))?;
    if catalog.schema != CAPABILITY_CATALOG_SCHEMA
        || catalog.application_protocol != APPLICATION_PROTOCOL
    {
        return Err("catalog_identity_mismatch".into());
    }
    let mut operations = BTreeSet::new();
    for op in &catalog.operations {
        if op.operation_id.is_empty()
            || op.input_contract.is_empty()
            || op.output_contract.is_empty()
            || !operations.insert(op.operation_id.as_str())
        {
            return Err("catalog_operation_invalid".into());
        }
    }
    let mut capabilities = BTreeSet::new();
    for capability in &catalog.capabilities {
        if !capabilities.insert(capability.capability_id.as_str())
            || capability
                .application_operation_ids
                .iter()
                .any(|id| !operations.contains(id.as_str()))
        {
            return Err("catalog_capability_invalid".into());
        }
    }
    Ok(catalog)
}
