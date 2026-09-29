//! Public value projections, not private state or authority objects.
use serde::{Deserialize, Serialize};
pub use crate::workflows::{ApplicationCapability, ApplicationCatalog, ApplicationOperation, CaseAttachment, CaseListProjection, CaseSummary, CaseVersionProjection, HandoffOfferReceipt, HandoffReceipt, TransitionReceipt, TransitionReceiptData, TransitionReceiptPayload, WorkCommit, WorkflowPatchReceipt};

/// Authorized Tenant machine pin returned by YAI Application. This is not a
/// YVEX runtime fact and does not establish reachability or client enrollment.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MachineAssetView {
    pub registration: MachineAssetRegistration,
    pub revocation: Option<MachineAssetRevocation>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MachineAssetRegistration {
    pub schema: String,
    pub asset_id: String,
    pub integrity_digest: String,
    pub tenant_id: String,
    pub device_identity: String,
    pub address: String,
    pub port: u16,
    pub management_user: String,
    pub host_public_key: String,
    pub approval_ref: String,
    pub approved_by_principal_id: String,
    pub approved_at_unix_ms: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MachineAssetRevocation {
    pub schema: String,
    pub asset_id: String,
    pub tenant_id: String,
    pub device_identity: String,
    pub revoked_at_unix_ms: u64,
    pub revoked_by_principal_id: String,
    pub reason: String,
    pub integrity_digest: String,
}

#[cfg(test)]
mod machine_projection_tests {
    use super::*;

    #[test]
    fn machine_pin_remains_yai_owned_and_distinct_from_runtime_state() {
        let value = serde_json::json!({"registration": {
            "schema":"yai.machine_asset.v1", "asset_id":"machine:one",
            "integrity_digest":"digest:one", "tenant_id":"tenant:one",
            "device_identity":format!("ssh-ed25519:sha256:{}", "a".repeat(64)),
            "address":"example.test", "port":2222, "management_user":"yvex",
            "host_public_key":format!("ssh-ed25519 {}", "A".repeat(64)),
            "approval_ref":"approval:one", "approved_by_principal_id":"principal:one",
            "approved_at_unix_ms":1
        }, "revocation":null});
        let pin: MachineAssetView = serde_json::from_value(value).unwrap();
        assert_eq!(pin.registration.asset_id, "machine:one");
        assert!(pin.revocation.is_none());
    }
}
/// Mutation receipt identity, never the private canonical CaseState layout.
#[cfg(test)]
mod receipt_tests {
    use super::*;
    #[test]
    fn commit_receipt_is_not_a_persisted_state_or_transition() {
        let value = serde_json::json!({"state":{"case_id":"case:test","generation":1},
            "transition":{"transition_id":"transition:test","payload":{"kind":"case_created","data":{}}}});
        let receipt: WorkCommit = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(receipt).unwrap(),value);
        for field in ["schema", "participants", "sources", "principal_participant_links"] {
            let mut private = value.clone(); private["state"][field] = serde_json::json!([]);
            assert!(serde_json::from_value::<WorkCommit>(private).is_err());
        }
        let mut private = value; private["transition"]["scope"] = serde_json::json!({"case_id":"case:test"});
        assert!(serde_json::from_value::<WorkCommit>(private).is_err());
    }
}

/// Public synthetic-check evidence, not the persisted carrier record.
/// Payload parameters are supported semantic contracts; this envelope never
/// carries carrier tokens, process identity, credential revisions or storage seals.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderProbeRunProjection<Request, Evidence, Qualification> {
    pub request: Request,
    pub evidence: Option<Evidence>,
    pub qualification: Option<Qualification>,
    pub failure_code: Option<String>,
    pub owner: ProviderProbeTiming,
}

/// The existing wire field `owner` exposes timing only, never carrier ownership.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderProbeTiming {
    pub started_at_unix_ms: u64,
}

#[cfg(test)]
mod probe_projection_tests {
    use super::*;
    type Probe = ProviderProbeRunProjection<serde_json::Value, serde_json::Value, serde_json::Value>;
    #[test]
    fn probe_projection_refuses_private_carrier_and_storage_fields() {
        let public = serde_json::json!({"request":{}, "evidence":null,
            "qualification":null,"failure_code":null,"owner":{"started_at_unix_ms":7}});
        assert!(serde_json::from_value::<Probe>(public.clone()).is_ok());
        for field in ["pid", "process_start_ticks", "boot_id", "token"] {
            let mut leaked = public.clone(); leaked["owner"][field] = serde_json::json!("private");
            assert!(serde_json::from_value::<Probe>(leaked).is_err());
        }
        for field in ["credential_revision", "integrity_digest", "principal_id"] {
            let mut leaked = public.clone(); leaked[field] = serde_json::json!("private");
            assert!(serde_json::from_value::<Probe>(leaked).is_err());
        }
    }
}
