//! Controlled public-contract checks; fixtures do not prove real Case admission.
use std::cell::RefCell;
use serde_json::{json, Value};
use yai_sdk::{client::{Client, Error}, workflows::*, *};

struct Peer {
    calls: RefCell<Vec<OperationRequest>>,
    unavailable: bool,
}
impl ClientTransport for &Peer {
    fn execute(&self, request: OperationRequest) -> Result<OperationResult, ClientError> {
        self.calls.borrow_mut().push(request.clone());
        if request.operation_ref == "skill.draft.save" {
            // Model a producer commit followed by a lost acknowledgement. The
            // caller can query the retained receipt, but never gets a retry.
            return Err(ClientError::after_dispatch("controlled_reply_loss".into()));
        }
        let data = match request.operation_ref.as_str() {
            "application.capabilities" => json!({"schema":CAPABILITY_CATALOG_SCHEMA,
                "application_protocol":APPLICATION_PROTOCOL,"capabilities":[],"operations":[
                    {"operation_id":"skill.list","meaning":"library","input_contract":"yai.skill_list_input.v1","output_contract":"yai.skill_catalog.v1","impact":"read","authority":"local_authenticated"},
                    {"operation_id":"skill.draft.save","meaning":"draft","input_contract":"yai.skill_draft_save_input.v1","output_contract":"yai.skill_draft.v1","impact":"canonical_mutation","authority":"tenant_owner"},
                    {"operation_id":"skill.mutation.get","meaning":"receipt","input_contract":"yai.skill_mutation_get_input.v1","output_contract":"yai.skill_mutation_observation.v1","impact":"read","authority":"tenant_owner"}]}),
            "skill.list" => json!([]),
            "skill.mutation.get" => {
                let request_ref=request.input["mutation_request_ref"].as_str().unwrap();
                if request_ref == "mutation:absent" {
                    json!({"schema":"yai.skill_mutation_observation.v1","tenant_ref":"tenant:test","request_ref":request_ref,"posture":"not_recorded","receipt":null})
                } else {
                    json!({"schema":"yai.skill_mutation_observation.v1","tenant_ref":"tenant:test","request_ref":request_ref,"posture":"recorded","receipt":{
                        "schema":"yai.skill_mutation_receipt.v1","tenant_ref":"tenant:test","request_ref":request_ref,
                        "principal_ref":"principal:controlled","operation_ref":"skill.draft.save","result":{"kind":"draft_saved","value":draft()}}})
                }
            },
            other => return Err(ClientError { code: format!("unexpected:{other}"), outcome_indeterminate: false }),
        };
        let unavailable=self.unavailable && request.operation_ref == "skill.list";
        Ok(OperationResult { operation_ref:request.operation_ref, correlation_ref:request.correlation_ref,
            result_state:if unavailable { ResultState::TransportUnavailable } else { ResultState::Success },
            data:if unavailable {None} else {Some(data)}, error:if unavailable {Some(OperationError {
                code:"controlled_unavailable".into(),message:"unavailable".into(),safe_message:"unavailable".into(),result_state:ResultState::TransportUnavailable,
            })} else {None},
        })
    }
}
fn draft() -> Value {
    json!({"schema":"yai.skill_draft.v1","tenant_ref":"tenant:test","skill_ref":"skill:reader","revision":1,
        "definition":{"name":"Reader","description":"Read admitted material","instructions":"Use only qualified available capabilities.","required_capabilities":[],"required_resources":[],"prerequisites":[],"scope":"case_work","provenance_refs":[]},
        "created_by_principal_ref":"principal:controlled","created_at_unix_ms":1,"content_digest":"sha256:controlled_fixture"})
}
#[test]
fn skills_library_is_a_typed_observed_array_and_unavailable_is_not_empty() {
    for unavailable in [false,true] {
        let peer=Peer{calls:RefCell::new(Vec::new()),unavailable};
        let client=Client::discover(&peer,"discovery").unwrap();
        let observed=client.skills().list("library",&SkillListInput{tenant_ref:"tenant:test".into()}).unwrap();
        if unavailable { assert_eq!(observed.state,ResultState::TransportUnavailable);assert!(observed.data.is_none()); }
        else { assert!(observed.data.unwrap().is_empty()); }
        assert_eq!(peer.calls.borrow().len(),2);
    }
}
#[test]
fn lost_skill_ack_is_recovered_by_exact_public_receipt_without_another_mutation() {
    let peer=Peer{calls:RefCell::new(Vec::new()),unavailable:false};
    let client=Client::discover(&peer,"discovery").unwrap();
    let input=SkillDraftSaveInput{tenant_ref:"tenant:test".into(),skill_ref:"skill:reader".into(),
        definition:serde_json::from_value(draft()["definition"].clone()).unwrap(),expected_revision:0};
    assert!(matches!(client.skills().save_draft("mutation:exact",&input),Err(Error::Transport(e)) if e.outcome_indeterminate));
    let observed=client.skills().mutation("receipt-read",&SkillMutationGetInput{tenant_ref:"tenant:test".into(),mutation_request_ref:"mutation:exact".into()}).unwrap().data.unwrap();
    assert_eq!(observed.posture,SkillMutationPosture::Recorded);
    let receipt=observed.receipt.unwrap();
    assert_eq!(receipt.request_ref,"mutation:exact");assert_eq!(receipt.operation_ref,"skill.draft.save");
    let SkillMutationResult::DraftSaved{value}=receipt.result else {panic!("wrong mutation kind")};
    assert_eq!(value.revision,1);assert_eq!(value.skill_ref,"skill:reader");
    let absent=client.skills().mutation("absent-read",&SkillMutationGetInput{tenant_ref:"tenant:test".into(),mutation_request_ref:"mutation:absent".into()}).unwrap().data.unwrap();
    assert_eq!(absent.posture,SkillMutationPosture::NotRecorded);assert!(absent.receipt.is_none());
    let calls=peer.calls.borrow();
    assert_eq!(calls.iter().filter(|r|r.operation_ref == "skill.draft.save").count(),1);
    assert_eq!(calls[1].correlation_ref,"mutation:exact");assert_eq!(calls[2].input["mutation_request_ref"],"mutation:exact");
}
#[test]
fn caller_principal_and_unknown_skill_result_cannot_be_smuggled_into_typed_contracts() {
    let mut input=json!({"tenant_ref":"tenant:test","skill_ref":"skill:reader","definition":draft()["definition"],"expected_revision":0});
    input["principal_ref"]=json!("principal:spoofed");
    assert!(serde_json::from_value::<SkillDraftSaveInput>(input).is_err());
    assert!(serde_json::from_value::<SkillMutationResult>(json!({"kind":"grant_created","value":{"allowed":true}})).is_err());
    assert!(serde_json::from_value::<SkillAvailability>(json!("granted")).is_err());
}
