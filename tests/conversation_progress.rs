//! Contract/recovery tests only; fixtures do not qualify a real producer.
use std::cell::RefCell;
use yai_sdk::{client::{Client, Error}, workflows::*, *};
struct Peer { calls: RefCell<Vec<OperationRequest>>, lose_send: bool }
impl ClientTransport for &Peer {
    fn execute(&self, request: OperationRequest) -> Result<OperationResult, ClientError> {
        self.calls.borrow_mut().push(request.clone());
        if request.operation_ref == "conversation.send" && self.lose_send { return Err(ClientError::after_dispatch("lost".into())); }
        let data = match request.operation_ref.as_str() {
            "application.capabilities" => { let mut v:serde_json::Value=serde_json::from_str(include_str!("../contract/operations.json")).unwrap();v["capabilities"]=serde_json::json!([]);v },
            "conversation.send" => serde_json::to_value(TextConversationSubmission::default()).unwrap(),
            "conversation.progress.get" => serde_json::to_value(ConversationProgressObservation { schema:"yai.conversation_progress.v1".into(), partial_output:Some("provisional".into()), availability:ConversationProgressAvailability::Live, ..Default::default() }).unwrap(),
            _ => panic!("unexpected operation")
        };
        Ok(OperationResult { operation_ref:request.operation_ref,correlation_ref:request.correlation_ref,result_state:ResultState::Success,error:None,data:Some(data) })
    }
}
#[test]
fn progressive_opt_in_preserves_old_send_and_exact_observation_recovery() {
    let peer=Peer{calls:RefCell::new(vec![]),lose_send:false};let client=Client::discover(&peer,"discover").unwrap();
    client.conversation().send_text("old",&TextConversationInput::default()).unwrap();
    client.conversation().send_progressive("live",&ProgressiveConversationInput{progressive_output:true,..Default::default()}).unwrap();
    let observed=client.conversation().progress("read",&ConversationProgressInput{request_ref:"request:exact".into(),after_sequence:17,stream_ref:Some("stream:exact".into()),..Default::default()}).unwrap().data.unwrap();
    assert_eq!(observed.partial_output.as_deref(),Some("provisional"));assert!(observed.execution.primary_result.is_none());
    let calls=peer.calls.borrow();assert!(calls[1].input.get("progressive_output").is_none());assert_eq!(calls[2].input["progressive_output"],true);
    assert_eq!(calls[3].input["request_ref"],"request:exact");assert_eq!(calls[3].input["after_sequence"],17);assert_eq!(calls[3].input["stream_ref"],"stream:exact");
}
#[test]
fn lost_send_is_not_replayed_by_the_public_client() {
    let peer=Peer{calls:RefCell::new(vec![]),lose_send:true};let client=Client::discover(&peer,"discover").unwrap();
    assert!(matches!(client.conversation().send_progressive("same",&ProgressiveConversationInput{progressive_output:true,..Default::default()}),Err(Error::Transport(e)) if e.outcome_indeterminate));
    client.conversation().progress("observe",&ConversationProgressInput{request_ref:"request:exact".into(),..Default::default()}).unwrap();
    assert_eq!(peer.calls.borrow().iter().filter(|r|r.operation_ref=="conversation.send").count(),1);
}
#[test]
fn pending_progress_keeps_explicit_no_result_without_promoting_partial_output() {
    let mut observation = serde_json::to_value(ConversationProgressObservation {
        schema: "yai.conversation_progress.v1".into(),
        partial_output: Some("provisional".into()),
        availability: ConversationProgressAvailability::Live,
        execution: ConversationObservation {
            identity: None,
            case_ref: "case:exact".into(), request_ref: "request:exact".into(),
            ..Default::default()
        },
        ..Default::default()
    }).unwrap();
    assert_eq!(observation["execution"]["primary_result"], serde_json::Value::Null);
    observation["execution"].as_object_mut().unwrap().remove("primary_result");
    let old: ConversationProgressObservation = serde_json::from_value(observation).unwrap();
    assert!(old.execution.primary_result.is_none());
    assert!(old.execution.work.is_none());
    assert_eq!(old.partial_output.as_deref(), Some("provisional"));
    assert_eq!(serde_json::to_value(old).unwrap()["execution"]["primary_result"], serde_json::Value::Null);
}

#[test]
fn public_progress_refuses_unknown_channels_and_fake_canonical_fields() {
    let mut event=serde_json::to_value(ConversationProgressEvent::default()).unwrap();event["kind"]=serde_json::json!("hidden_reasoning");assert!(serde_json::from_value::<ConversationProgressEvent>(event).is_err());
    let mut value=serde_json::to_value(ConversationProgressObservation::default()).unwrap();value["canonical_partial"]=serde_json::json!(true);assert!(serde_json::from_value::<ConversationProgressObservation>(value).is_err());
    assert!(serde_json::from_str::<ConversationProgressAvailability>("\"zero\"").is_err());
}
