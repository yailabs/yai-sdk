//! Public projections and backwards reads; no private Core import or live API.
use serde_json::json;
use yai_sdk::workflows::*;

#[test]
fn protocol_profiles_are_typed_closed_and_never_contain_credentials(){
    for protocol in ["open_ai_chat","anthropic_messages","google_generate_content","cohere_chat","bedrock_converse"]{
        let profile:ProviderHttpProfile=serde_json::from_value(json!({"protocol":protocol,"remote_inference_opt_in":false})).unwrap();
        assert!(profile.catalog_path.is_none());assert!(profile.remote_inference_opt_in==Some(false));
        assert!(serde_json::from_value::<ProviderHttpProfile>(json!({"protocol":protocol,"api_key":"never_a_public_profile_field"})).is_err());
    }
    assert!(serde_json::from_value::<ProviderAuthentication>(json!({"kind":"aws_sig_v4","region":"us-east-1","secret_access_key":"not_public"})).is_err());
}

#[test]
fn text_only_automatic_v2_is_explicit_and_does_not_synthesize_work(){
    let input:TextConversationInput=serde_json::from_value(json!({"execution":"automatic_v2","case_ref":"case:a","participant_ref":"participant:author",
        "thread_ref":"thread:a","submission_ref":"send:new","expected_generation":1,"parts":[]})).unwrap();
    assert_eq!(input.execution,Some(ConversationExecutionMode::AutomaticV2));
    let observation:AutomaticConversationObservation=serde_json::from_value(json!({"binding_ref":"binding:exact",
        "answer_limits":{"max_input_units":4096,"max_output_tokens":96},"consequence":"answer_only_text_profile_no_operations"})).unwrap();
    assert!(observation.limits.is_none());assert_eq!(observation.answer_limits.unwrap().max_output_tokens,96);
    let historical:AutomaticConversationObservation=serde_json::from_value(json!({"binding_ref":"binding:old","limits":{"invocations":2,"operations":1,"effects":0,"max_input_units":4096},"consequence":"answer_only"})).unwrap();
    assert!(historical.answer_limits.is_none());assert!(historical.limits.is_some());
}

#[test]
fn independent_setup_and_laboratory_preserve_unknown_and_historical_fields(){
    let setup:ProviderCaseSetupInput=serde_json::from_value(json!({"case_ref":"case:a","expected_generation":1,"author_participant_ref":"participant:author",
        "executor_participant_ref":"participant:model","candidates":[],"answer_limits":{"max_input_units":4096,"max_output_tokens":96},"failover_policy":"safe_only","max_attempts_per_turn":1})).unwrap();
    assert!(setup.conversation_limits.is_none());assert!(setup.answer_limits.is_some());
    assert!(serde_json::from_value::<ProviderCaseSetupInput>(json!({"case_ref":"case:a","principal_id":"forged"})).is_err());
    let sample:ProviderProbeSample=serde_json::from_value(json!({"stage":"text","protocol":"cohere_chat","request_identity":"sha256:exact","request_bytes":1,
        "response_identity":null,"response_bytes":null,"http_status":null,"started_at_unix_ms":1,"caller_elapsed_micros":1,"input_tokens":null,"output_tokens":null,
        "total_tokens":null,"conservative_wire_input_units":1,"first_public_fragment_micros":null,"producer_compute_micros":null,"reported_model_id":null,"identity_assurance":"request_bound"})).unwrap();
    assert!(sample.input_tokens.is_none());assert!(sample.producer_compute_micros.is_none());
    assert_eq!(serde_json::to_value(sample).unwrap()["input_tokens"],json!(null));
    let compare:ProviderLabCompareInput=serde_json::from_value(json!({"executions":[{"case_ref":"case:a","author_participant_ref":"participant:author","submission_ref":"send:original"}]})).unwrap();
    assert!(compare.runs.is_none());assert_eq!(compare.executions.unwrap()[0].submission_ref,"send:original");
}
