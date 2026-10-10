//! Public value projections, not private state or authority objects.
use serde::{Deserialize, Serialize};
pub use crate::workflows::{ApplicationCapability, ApplicationCatalog, ApplicationOperation, CaseAttachment, CaseListProjection, CaseSummary, CaseVersionProjection, HandoffOfferReceipt, HandoffReceipt, TransitionReceipt, TransitionReceiptData, TransitionReceiptPayload, WorkCommit, WorkflowPatchReceipt};

/// Existing v1 inspection input with an optional exact Operation fence.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionInspectInput {
    pub case_ref: String,
    pub participant_ref: String,
    pub decision_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_ref: Option<String>,
}

impl crate::client::Operation for DecisionInspectInput {
    type Output = DecisionTrajectory;
    const ID: &'static str = "decision.trajectory.inspect";
    const INPUT_CONTRACT: Option<&'static str> = Some("yai.decision_trajectory_inspect_input.v1");
    const OUTPUT_CONTRACT: Option<&'static str> = Some("yai.cognitive_decision_trajectory.v1");
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionRuleProvenance {
    pub binding_id: String,
    pub artifact_id: String,
    pub policy_ir_rule_id: String,
    pub source_id: String,
    pub fact_refs: Vec<String>,
    pub source_locations: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionValidityContract {
    pub mode: String,
    pub valid_from_unix_ms: Option<u64>,
    pub refresh_after_unix_ms: Option<u64>,
    pub expires_at_unix_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionPolicyValidity {
    pub binding_id: String,
    pub lineage_id: String,
    pub artifact_id: String,
    pub contract: DecisionValidityContract,
    pub posture: String,
    pub reason: String,
    pub revoke_event_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionAuthorityEvaluation {
    pub subject: String,
    pub participant_id: Option<String>,
    pub required_roles: Vec<String>,
    pub observed_roles: Vec<String>,
    pub eligible_participant_ids: Vec<String>,
    pub satisfied: bool,
    pub contributing_rules: Vec<DecisionRuleProvenance>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionObligationEvaluation {
    pub obligation: String,
    pub status: String,
    pub evidence_refs: Vec<String>,
    pub contributing_rules: Vec<DecisionRuleProvenance>,
}

/// Retained v1/v2/v3 evaluated basis. Decoding is not admission or a Grant.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionBasis {
    pub schema: String,
    pub basis_id: String,
    pub integrity_digest: String,
    pub case_id: String,
    pub tenant_id: Option<String>,
    pub evaluated_case_generation: u64,
    #[serde(default)]
    pub authority_evaluated_at_unix_ms: u64,
    #[serde(default)]
    pub policy_validity: Vec<DecisionPolicyValidity>,
    pub earliest_policy_expiry_unix_ms: Option<u64>,
    pub operation_id: String,
    pub operation_digest: String,
    pub operation_kind: String,
    pub proposer_participant_id: String,
    pub resource_attachment_id: String,
    pub resource_kind: String,
    pub effective_policy_id: String,
    pub effective_policy_digest: String,
    pub materializer_version: String,
    pub policy_binding_refs: Vec<String>,
    pub policy_artifact_refs: Vec<String>,
    pub matched_rule_refs: Vec<String>,
    pub contributing_provenance: Vec<DecisionRuleProvenance>,
    pub mechanical_posture: String,
    pub operation_restriction: String,
    pub review_required: bool,
    pub authority: Vec<DecisionAuthorityEvaluation>,
    pub obligations: Vec<DecisionObligationEvaluation>,
    pub final_posture: String,
    pub final_reason: String,
    pub review_action_ref: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DecisionSource {
    pub policy_id: String,
    pub owner_participant_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DecisionProjection {
    pub schema: Option<String>,
    pub decision_id: String,
    pub decision_digest: Option<String>,
    pub operation_id: String,
    pub operation_digest: Option<String>,
    pub source: Option<DecisionSource>,
    pub outcome: String,
    pub reason: String,
    pub basis_refs: Vec<String>,
    pub decision_basis: Option<DecisionBasis>,
    pub decided_at_case_generation: Option<u64>,
}

/// Typed normative facts inside the already released trajectory projection.
/// Other historical families remain public JSON; clients do not reconstruct them.
#[derive(Clone, Debug, Deserialize)]
pub struct DecisionTrajectory {
    pub schema: String,
    pub trajectory_id: String,
    pub case_id: String,
    pub participant_id: String,
    pub decision_transition_id: String,
    pub decision_generation: u64,
    pub decision: DecisionProjection,
    pub pre_decision: serde_json::Value,
    pub selected_operation: Option<crate::workflows::CaseOperation>,
    pub missingness: Vec<String>,
    pub readiness: serde_json::Value,
}

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
