//! Public representation only. Authority and actual file effects are Core proof.
use yai_sdk::workflows::*;
use serde_json::json;

#[test]
fn baseline_content_is_an_explicit_compatible_retained_read() {
    let input = MaterialInspectInput::default();
    let wire = serde_json::to_value(&input).unwrap();
    assert!(wire.get("include_baseline_content").is_none());
    let requested = MaterialInspectInput { include_baseline_content: Some(true), ..input };
    assert_eq!(serde_json::to_value(requested).unwrap()["include_baseline_content"], true);
    let old_baseline = serde_json::to_value(MaterialReadEvidence::default()).unwrap();
    assert!(old_baseline.get("content").is_none(), "old closed clients receive their unchanged shape");
    let value: MaterialReadContent = serde_json::from_value(json!({"posture":"retained", "text":"exact\nè"})).unwrap();
    assert!(matches!(value, MaterialReadContent::Retained { text } if text == "exact\nè"));
    assert!(serde_json::from_value::<MaterialReadContent>(json!({"posture":"retained", "text":"x", "live_path":"/private"})).is_err());
    assert!(serde_json::from_value::<MaterialReadContent>(json!({"posture":"model_reconstructed", "text":"x"})).is_err());
}

#[test]
fn original_submission_is_not_request_alias_and_legacy_absence_is_explicit() {
    let identity: ConversationIdentity = serde_json::from_value(json!({
        "original_submission_ref":"send:client-a", "submission_posture":"retained",
        "author_participant_ref":"participant:author", "executor_participant_ref":"participant:model",
        "thread_ref":"thread:original"
    })).unwrap();
    assert_eq!(identity.original_submission_ref.as_deref(), Some("send:client-a"));
    let legacy: ConversationIdentity = serde_json::from_value(json!({
        "original_submission_ref":null, "submission_posture":"unavailable",
        "author_participant_ref":"participant:author", "executor_participant_ref":"participant:model", "thread_ref":"thread:original"
    })).unwrap();
    assert!(legacy.original_submission_ref.is_none());
    let input = ConversationRequestGetInput { execution:ConversationRequestReference {
        domain:ConversationRequestDomain::CognitiveComposition, request_ref:"request:canonical".into()
    }, ..Default::default() };
    let wire=serde_json::to_value(input).unwrap();
    assert_eq!(wire["execution"]["request_ref"], "request:canonical");
    assert!(wire["execution"].get("submission_ref").is_none());
    let mut extra=serde_json::to_value(identity).unwrap();extra["grant"]=json!("fabricated");
    assert!(serde_json::from_value::<ConversationIdentity>(extra).is_err());
}

#[test]
fn material_provenance_keeps_unverified_source_and_no_receipt_as_no_effect_claim() {
    let mut value=serde_json::to_value(MaterialProvenanceObservation::default()).unwrap();
    value["baseline_posture"]=json!("unavailable");value["posture"]=json!("proposed");
    value["source"]=json!({"source_ref":"source:independent", "revision_ref":null,"material_revision_ref":null,
        "source_resource_ref":null,"source_configuration_digest":null,"posture":"unverified","reason":"no exact basis"});
    let observed:MaterialProvenanceObservation=serde_json::from_value(value.clone()).unwrap();
    assert!(observed.receipt.is_none());assert!(observed.baseline.is_none());
    assert_eq!(observed.source.unwrap().posture,MaterialSourcePosture::Unverified);
    value["private_key"]=json!("must-not-project");
    assert!(serde_json::from_value::<MaterialProvenanceObservation>(value).is_err());
    assert!(serde_json::from_str::<MaterialEffectPosture>("\"model_claimed_success\"").is_err());
    assert!(serde_json::from_str::<MaterialSourcePosture>("\"same_filename\"").is_err());
}
