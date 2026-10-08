//! Representation/correlation only; this does not qualify Core sufficiency or a model.
use std::cell::RefCell;
use yai_sdk::{client::{Client, Error}, workflows::*, *};

struct Peer { calls: RefCell<Vec<OperationRequest>>, lost: bool }
impl ClientTransport for &Peer {
    fn execute(&self, request: OperationRequest) -> Result<OperationResult, ClientError> {
        let discovery = request.operation_ref == "application.capabilities";
        self.calls.borrow_mut().push(request.clone());
        if !discovery && self.lost { return Err(ClientError::after_dispatch("lost".into())); }
        let data = if discovery {
            let mut catalog: serde_json::Value = serde_json::from_str(include_str!("../contract/operations.json")).unwrap();
            catalog["capabilities"] = serde_json::json!([]); catalog
        } else if request.operation_ref == "conversation.send" {
            serde_json::to_value(TextConversationSubmission::default()).unwrap()
        } else {
            serde_json::to_value(ConversationObservation::default()).unwrap()
        };
        Ok(OperationResult { operation_ref:request.operation_ref, correlation_ref:request.correlation_ref,
            result_state:ResultState::Success, error:None, data:Some(data) })
    }
}

#[test]
fn context_variants_use_existing_operations_without_changing_old_inputs() {
    let peer = Peer { calls:RefCell::new(vec![]), lost:false };
    let client = Client::discover(&peer, "discovery").unwrap();
    client.conversation().send_text("old-send", &TextConversationInput::default()).unwrap();
    client.conversation().send_with_search("task-send", &TaskConversationInput {
        memory_search_mode:MemorySearchMode::Fast, ..Default::default()
    }).unwrap();
    client.conversation().get_context("observe-only", &ConversationContextGetInput {
        include_context:true, ..Default::default()
    }).unwrap();
    let calls = peer.calls.borrow();
    assert_eq!(calls[1].operation_ref, "conversation.send");
    assert!(calls[1].input.get("memory_search_mode").is_none());
    assert_eq!(calls[2].input["memory_search_mode"], "fast");
    assert_eq!(calls[3].operation_ref, "execution.get");
    assert_eq!(calls[3].input["include_context"], true);
    assert!(serde_json::to_value(ConversationGetInput::default()).unwrap().get("include_context").is_none());
}

#[test]
fn lost_context_send_never_retries_or_changes_search_mode() {
    let peer = Peer { calls:RefCell::new(vec![]), lost:true };
    let client = Client::discover(&peer, "discovery").unwrap();
    assert!(matches!(client.conversation().send_with_search("immutable", &TaskConversationInput::default()),
        Err(Error::Transport(e)) if e.outcome_indeterminate));
    assert_eq!(peer.calls.borrow().len(), 2);
}

#[test]
fn context_diagnostics_preserve_unavailable_and_refuse_fabricated_confidence_or_secret() {
    let mut value = serde_json::to_value(ContextPreparationObservation::default()).unwrap();
    value["source"] = serde_json::json!("deterministic");
    value["finite_dispatch"] = serde_json::json!("outcome_unavailable");
    let observed: ContextPreparationObservation = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(observed.source, ContextSelectionSource::Deterministic);
    assert_eq!(observed.finite_dispatch, Some(FiniteDispatchPosture::OutcomeUnavailable));
    assert!(observed.finite_ranking.is_none());
    value["bearer"] = serde_json::json!("never-public");
    assert!(serde_json::from_value::<ContextPreparationObservation>(value).is_err());
    assert!(serde_json::from_str::<ContextSelectionSource>("\"calibrated_confidence\"").is_err());
}
