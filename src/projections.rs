//! Public value projections, not private state or authority objects.
use serde::{Deserialize, Serialize};
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
