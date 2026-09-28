//! Public value projections, not private state or authority objects.
use serde::{Deserialize, Serialize};
/// Mutation receipt identity, never the private canonical CaseState layout.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaseVersionProjection {
    pub case_id: String,
    pub generation: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkCommit {
    pub state: CaseVersionProjection,
    pub transition: TransitionReceipt,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransitionReceipt {
    pub transition_id: String,
    pub payload: TransitionReceiptPayload,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransitionReceiptPayload {
    pub kind: String,
    pub data: TransitionReceiptData,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransitionReceiptData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<WorkflowPatchReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer: Option<HandoffOfferReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<HandoffReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline: Option<HandoffReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<HandoffReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation: Option<HandoffReceipt>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowPatchReceipt {
    pub patch_id: String,
    pub base_effective_topology_digest: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffOfferReceipt {
    pub handoff_id: String,
    pub source_case_id: String,
    pub target_case_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffReceipt {
    pub handoff_id: String,
}
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CaseListProjection {
    pub cases: Vec<CaseSummary>,
    pub authority: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CaseSummary {
    pub case_ref: String,
    pub display_name: String,
    pub case_status: String,
    pub generation: u64,
    pub updated_at_unix_ms: Option<u64>,
    pub participant_count: usize,
    pub source_count: usize,
    pub resource_count: usize,
    pub pending_review_count: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CaseAttachment {
    pub case_ref: String,
    pub case_status: String,
    pub generation: u64,
    pub authenticated_principal: String,
    pub participant_ref: String,
    pub thread_ref: Option<String>,
    pub attachment: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationCatalog {
    pub schema: String,
    pub application_protocol: String,
    pub operations: Vec<ApplicationOperation>,
    pub capabilities: Vec<ApplicationCapability>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationOperation {
    pub operation_id: String,
    pub meaning: String,
    pub input_contract: String,
    pub output_contract: String,
    pub impact: String,
    pub authority: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationCapability {
    pub capability_id: String,
    pub meaning: String,
    pub application_posture: String,
    pub application_operation_ids: Vec<String>,
    pub application_deferred_reason: Option<String>,
    pub studio_posture: String,
}
