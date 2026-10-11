use yai_sdk::workflows::*;
use serde_json::json;

#[test]
fn automatic_is_explicit_additive_and_not_legacy_effect_upgrade() {
    let old = serde_json::to_value(TextConversationInput::default()).unwrap();
    assert!(old.get("execution").is_none());
    assert!(old.get("work_limits").is_none());
    let automatic = TextConversationInput { execution:Some(ConversationExecutionMode::Automatic), ..Default::default() };
    assert_eq!(automatic.execution, Some(ConversationExecutionMode::Automatic));
    assert!(automatic.intent.is_none());
    assert_eq!(serde_json::to_value(&automatic).unwrap()["execution"], "automatic");
    assert!(serde_json::from_value::<ConversationExecutionMode>(json!("unlimited")).is_err());
}

#[test]
fn answer_only_is_not_authority_or_attestation() {
    let old = serde_json::to_value(ConversationObservation::default()).unwrap();
    assert!(old.get("automatic_execution").is_none());
    let observation: AutomaticConversationObservation = serde_json::from_value(json!({
        "binding_ref":"case-cognitive-binding:exact", "limits":{
            "invocations":2,"operations":1,"effects":0,"max_input_units":8192,"max_output_tokens":96
        }, "consequence":"answer_only"
    })).unwrap();
    assert_eq!(observation.consequence, AutomaticConversationConsequence::AnswerOnly);
    assert!(serde_json::from_value::<AutomaticConversationConsequence>(json!("model_claimed_allowed")).is_err());
}
