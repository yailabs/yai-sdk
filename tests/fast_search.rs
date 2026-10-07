//! Client projection controls only; no producer, model or Core qualification.
use std::cell::RefCell;
use yai_sdk::{client::{Client, Error}, workflows::*, *};

struct Peer { calls: RefCell<Vec<String>>, lost: bool }
impl ClientTransport for &Peer {
    fn execute(&self, request: OperationRequest) -> Result<OperationResult, ClientError> {
        self.calls.borrow_mut().push(request.operation_ref.clone());
        if self.lost && request.operation_ref != "application.capabilities" {
            return Err(ClientError::after_dispatch("lost".into()));
        }
        let data = if request.operation_ref == "application.capabilities" {
            let mut catalog:serde_json::Value = serde_json::from_str(include_str!("../contract/operations.json")).unwrap();
            catalog["capabilities"] = serde_json::json!([]);
            catalog
        } else {
            serde_json::to_value(FastSearchResult {
                schema:"yai.fast_search_result.v1".into(), posture:FastSearchPosture::DeterministicFallback,
                reason:"finite_compute_budget_exceeded".into(), ..Default::default()
            }).unwrap()
        };
        Ok(OperationResult { operation_ref:request.operation_ref, correlation_ref:request.correlation_ref,
            result_state:ResultState::Success, error:None, data:Some(data) })
    }
}

#[test]
fn finite_result_is_typed_and_budget_refusal_never_becomes_a_score() {
    let peer = Peer { calls:RefCell::new(Vec::new()), lost:false };
    let client = Client::discover(&peer,"discovery").unwrap();
    let result = client.cognitive().fast_search("read:one",&FastSearchInput::default()).unwrap();
    let value = result.data.unwrap();
    assert_eq!(value.posture,FastSearchPosture::DeterministicFallback);
    assert!(value.ranking.is_none() && value.scores.is_empty());
    assert_eq!(value.reason,"finite_compute_budget_exceeded");
    assert_eq!(peer.calls.borrow().as_slice(),["application.capabilities","semantic.fast_search.execute"]);
}

#[test]
fn lost_finite_response_is_indeterminate_without_sdk_retry() {
    let peer = Peer { calls:RefCell::new(Vec::new()), lost:true };
    let client = Client::discover(&peer,"discovery").unwrap();
    assert!(matches!(client.cognitive().fast_search("read:one",&FastSearchInput::default()),
        Err(Error::Transport(error)) if error.outcome_indeterminate));
    assert_eq!(peer.calls.borrow().len(),2);
}

#[test]
fn finite_projection_excludes_credentials_and_unknown_postures() {
    let mut value = serde_json::to_value(FastSearchResult::default()).unwrap();
    value["private_key"] = serde_json::json!("never-public");
    assert!(serde_json::from_value::<FastSearchResult>(value).is_err());
    let mut value = serde_json::to_value(FiniteQualifyInput::default()).unwrap();
    value["expected_generation"] = serde_json::json!(7);
    value["retry"] = serde_json::json!(true);
    assert!(serde_json::from_value::<FiniteQualifyInput>(value).is_err());
    assert!(serde_json::from_str::<FiniteDispatchPosture>("\"retry_safe\"").is_err());
    assert!(serde_json::from_str::<FastSearchPosture>("\"model_confident\"").is_err());
}
