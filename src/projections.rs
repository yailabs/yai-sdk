//! Public value projections, not private state or authority objects.
use serde::{Deserialize, Serialize};
pub use crate::workflows::{ApplicationCapability, ApplicationCatalog, ApplicationOperation, CaseAttachment, CaseListProjection, CaseSummary, CaseVersionProjection, HandoffOfferReceipt, HandoffReceipt, TransitionReceipt, TransitionReceiptData, TransitionReceiptPayload, WorkCommit, WorkflowPatchReceipt};
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
