//! Independent decoding of an actual controlled public Core Decision, not admission.
use serde_json::{json, Value};
use yai_sdk::projections::{DecisionInspectInput, DecisionProjection, DecisionTrajectory};

#[test]
fn public_core_v3_basis_preserves_exact_historical_contributions() {
    let raw: Value = serde_json::from_str(include_str!("fixtures/delegated-decision-v3.json")).unwrap();
    let decision: DecisionProjection = serde_json::from_value(raw.clone()).unwrap();
    let basis = decision.decision_basis.unwrap();
    assert_eq!(basis.schema, "yai.decision_basis.v3");
    assert_eq!(decision.outcome, "require_review");
    assert_eq!(basis.operation_id, decision.operation_id);
    assert_eq!(basis.operation_digest, decision.operation_digest.unwrap());
    assert_eq!(basis.proposer_participant_id, "participant:model");
    assert_eq!(basis.mechanical_posture, "satisfied");
    assert_eq!(basis.policy_validity[0].contract.mode, "unbounded");
    assert!(basis.policy_validity[0].contract.expires_at_unix_ms.is_none());
    assert_eq!(serde_json::to_value(&basis).unwrap()["contributing_provenance"],
        raw["decision_basis"]["contributing_provenance"]);
    assert!(!basis.policy_binding_refs.is_empty());
    assert!(!basis.policy_artifact_refs.is_empty());
    assert!(!basis.matched_rule_refs.is_empty());
    for rule in &basis.contributing_provenance {
        assert!(basis.policy_binding_refs.contains(&rule.binding_id));
        assert!(basis.policy_artifact_refs.contains(&rule.artifact_id));
        assert!(!rule.source_id.is_empty());
        assert!(!rule.source_locations.is_empty());
    }
}

#[test]
fn legacy_missing_basis_and_explicit_null_never_fabricate_policy() {
    let legacy = json!({"decision_id":"decision:legacy", "operation_id":"operation:legacy",
        "outcome":"deny", "reason":"legacy", "basis_refs":[]});
    for explicit_null in [false, true] {
        let mut value = legacy.clone();
        if explicit_null { value["decision_basis"] = Value::Null; }
        let decoded: DecisionProjection = serde_json::from_value(value).unwrap();
        assert!(decoded.decision_basis.is_none());
        assert!(decoded.operation_digest.is_none());
        assert_eq!(decoded.outcome, "deny");
    }
    let mut source = legacy;
    source["source"] = json!({"policy_id":"policy:legacy", "owner_participant_id":"participant:owner"});
    assert_eq!(serde_json::from_value::<DecisionProjection>(source).unwrap().source.unwrap().policy_id, "policy:legacy");
}

#[test]
fn old_and_fenced_inspection_inputs_decode_without_new_operation() {
    use yai_sdk::client::Operation;
    assert_eq!(DecisionInspectInput::ID, "decision.trajectory.inspect");
    assert_eq!(DecisionInspectInput::INPUT_CONTRACT, Some("yai.decision_trajectory_inspect_input.v1"));
    assert_eq!(DecisionInspectInput::OUTPUT_CONTRACT, Some("yai.cognitive_decision_trajectory.v1"));
    let raw = json!({"case_ref":"case:a", "participant_ref":"participant:author", "decision_ref":"decision:a"});
    let old: DecisionInspectInput = serde_json::from_value(raw.clone()).unwrap();
    assert!(old.operation_ref.is_none());
    assert_eq!(serde_json::to_value(old).unwrap(), raw);
    let mut fenced = raw;
    fenced["operation_ref"] = json!("operation:a");
    assert_eq!(serde_json::from_value::<DecisionInspectInput>(fenced).unwrap().operation_ref.as_deref(), Some("operation:a"));
}

#[test]
fn delegated_observer_is_not_the_proposer_and_unavailable_is_not_allow() {
    let decision: Value = serde_json::from_str(include_str!("fixtures/delegated-decision-v3.json")).unwrap();
    let trajectory: DecisionTrajectory = serde_json::from_value(json!({
        "schema":"yai.cognitive_decision_trajectory.v1", "trajectory_id":"trajectory:public",
        "case_id":"case:material-qualification", "participant_id":"participant:human",
        "decision_transition_id":"transition:public", "decision_generation":37,
        "decision":decision, "pre_decision":{}, "missingness":[], "readiness":{}})).unwrap();
    assert_ne!(trajectory.participant_id, trajectory.decision.decision_basis.unwrap().proposer_participant_id);
    assert!(trajectory.selected_operation.is_none());
    // Incomplete/malformed normative data is a decode error, not invented permission.
    let mut wrong = decision;
    wrong["decision_basis"]["contributing_provenance"] = json!([{"source_id":"source:unbacked"}]);
    assert!(serde_json::from_value::<DecisionProjection>(wrong).is_err());
}
