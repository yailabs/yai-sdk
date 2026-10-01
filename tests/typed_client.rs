use std::cell::RefCell;
use serde::Serialize;
use yai_sdk::{client::{Client, Error, Operation}, *};

#[derive(Serialize)]
struct ListCases {}
impl Operation for ListCases {
    type Output = projections::CaseListProjection;
    const ID: &'static str = "case.list";
}
struct Mock { calls: RefCell<Vec<String>>, mode: &'static str }
impl ClientTransport for &Mock {
    fn execute(&self, request: OperationRequest) -> Result<OperationResult, ClientError> {
        self.calls.borrow_mut().push(request.operation_ref.clone());
        if request.operation_ref != "application.capabilities" && self.mode == "lost" {
            return Err(ClientError::after_dispatch("lost".into()));
        }
        let mut result = OperationResult {
            operation_ref: request.operation_ref.clone(), correlation_ref: request.correlation_ref,
            result_state: ResultState::Success, error: None,
            data: Some(if request.operation_ref == "application.capabilities" {
                serde_json::json!({"schema": CAPABILITY_CATALOG_SCHEMA,
                    "application_protocol": APPLICATION_PROTOCOL, "capabilities":[],
                    "operations": if self.mode == "unsupported" { vec![] } else { vec![serde_json::json!({
                        "operation_id":"case.list", "meaning":"visible cases", "input_contract":"list",
                        "output_contract":"cases", "impact":"read", "authority":"local_authenticated"})] }})
            } else { serde_json::json!({"cases":[], "authority":"current_disclosure"}) }),
        };
        if request.operation_ref != "application.capabilities" {
            match self.mode {
                "refused" => { result.result_state = ResultState::Unauthorized; result.data = None;
                    result.error = Some(OperationError { code:"denied".into(), message:"denied".into(),
                        safe_message:"denied".into(), result_state:ResultState::Unauthorized }); }
                "foreign" => result.correlation_ref = "other".into(),
                "malformed" => result.data = Some(serde_json::json!({"private_state":{}})),
                "unavailable" => { result.result_state = ResultState::CorePending;
                    result.data = Some(serde_json::json!({"availability":"producer_unavailable"})); }
                "partial" => result.result_state = ResultState::Partial,
                _ => {}
            }
        }
        Ok(result)
    }
}
fn mock(mode: &'static str) -> Mock { Mock { calls: RefCell::new(vec![]), mode } }

#[test]
fn typed_read_preserves_public_projection_and_correlation() {
    let transport = mock("ok");
    let client = Client::discover(&transport, "discovery:1").unwrap();
    assert!(client.supports("case.list"));
    let response = client.execute("read:1", &ListCases {}).unwrap();
    assert_eq!(response.state, ResultState::Success);
    assert_eq!(response.correlation_ref, "read:1");
    assert_eq!(response.data.unwrap().authority, "current_disclosure");
}
#[test]
fn missing_capability_refuses_before_dispatch() {
    let transport = mock("unsupported");
    let client = Client::discover(&transport, "discovery:1").unwrap();
    assert!(matches!(client.execute("read:1", &ListCases {}), Err(Error::UnsupportedOperation(_))));
    assert_eq!(transport.calls.borrow().len(), 1);
}
#[test]
fn semantic_refusal_is_not_transport_loss() {
    let transport = mock("refused");
    let client = Client::discover(&transport, "discovery:1").unwrap();
    let response = client.execute("read:1", &ListCases {}).unwrap();
    assert_eq!(response.state, ResultState::Unauthorized);
    assert!(response.data.is_none());
    assert_eq!(response.error.unwrap().code, "denied");
}
#[test]
fn post_dispatch_loss_remains_indeterminate_without_retry() {
    let transport = mock("lost");
    let client = Client::discover(&transport, "discovery:1").unwrap();
    let Err(Error::Transport(error)) = client.execute("read:1", &ListCases {}) else { panic!() };
    assert!(error.outcome_indeterminate);
    assert_eq!(transport.calls.borrow().len(), 2);
}

#[test]
fn activation_contract_preserves_pending_identity_and_rejects_credentials() {
    use workflows::{ProductActivationStartInput, ProductActivationPollInput, ProductAuthObservation, ActivationPosture};
    let start=serde_json::to_value(ProductActivationStartInput {name:"actual installation".into()}).unwrap();
    assert_eq!(start,serde_json::json!({"name":"actual installation"}));
    assert!(serde_json::from_value::<ProductActivationStartInput>(serde_json::json!({"email":"existing@example.invalid"})).is_err());
    assert!(serde_json::from_value::<ProductActivationPollInput>(serde_json::json!({"activation_ref":"activation:one","access_token":"not-a-client-input"})).is_err());
    let mut value=serde_json::json!({"schema":"yai.product_auth.v1","session":"activation_pending","service":"reachable",
        "account_ref":null,"commercial_installation_ref":null,"access":null,"verification_refusal":null,
        "online_access_refused":false,"licensed_progress_allowed":false,"credential_storage":"native_os_credential_store",
        "activation":{"activation_ref":"activation:one","posture":"indeterminate","user_code":"1234-ABCD",
            "verification_uri":"https://commercial.invalid/activate/","verification_uri_complete":"https://commercial.invalid/activate/?code=1234-ABCD",
            "expires_at_unix_ms":600000,"poll_interval_ms":5000,"next_poll_at_unix_ms":10000}});
    let observed:ProductAuthObservation=serde_json::from_value(value.clone()).unwrap();
    assert_eq!(observed.activation.unwrap().posture,ActivationPosture::Indeterminate);
    value["access_token"]=serde_json::json!("must-never-cross-the-client-contract");
    assert!(serde_json::from_value::<ProductAuthObservation>(value).is_err());
}
#[test]
fn wrong_identity_and_malformed_success_are_not_successes() {
    for mode in ["foreign", "malformed"] {
        let transport = mock(mode);
        let client = Client::discover(&transport, "discovery:1").unwrap();
        assert!(matches!(client.execute("read:1", &ListCases {}), Err(Error::InvalidResponse { .. })));
        assert_eq!(transport.calls.borrow().len(), 2);
    }
}

#[test]
fn generated_workflow_refuses_foreign_schema_before_dispatch() {
    let transport = mock("ok");
    let client = Client::discover(&transport, "discovery:1").unwrap();
    assert!(matches!(client.cases().list("read:1", &workflows::CaseListInput { tenant_id: None }),
        Err(Error::ContractMismatch(_))));
    assert_eq!(transport.calls.borrow().len(), 1);
}

#[test]
fn released_catalog_requires_exact_contracts_but_allows_additive_operations() {
    let catalog: serde_json::Value = serde_json::from_str(include_str!("../contract/operations.json")).unwrap();
    let mut data = catalog;
    data["capabilities"] = serde_json::json!([]);
    let mut response = OperationResult { operation_ref: "application.capabilities".into(),
        result_state: ResultState::Success, correlation_ref: "catalog".into(),
        data: Some(data), error: None };
    let expected = response.data.as_ref().unwrap()["operations"].as_array().unwrap().len();
    assert_eq!(conformance::validate_released_catalog(&response).unwrap(), expected);
    response.data.as_mut().unwrap()["operations"].as_array_mut().unwrap().push(serde_json::json!({
        "operation_id":"future.read", "meaning":"future", "input_contract":"future.in.v1",
        "output_contract":"future.out.v1", "impact":"read", "authority":"case_disclosure"}));
    assert_eq!(conformance::validate_released_catalog(&response).unwrap(), expected);
    response.data.as_mut().unwrap()["operations"][0]["output_contract"] = "foreign.v2".into();
    assert!(conformance::validate_released_catalog(&response).is_err());
}

#[test]
fn public_numeric_and_temporal_projection_preserves_signed_rank_and_cut() {
    let candidate: workflows::RecallCandidate = serde_json::from_value(serde_json::json!({
        "source_ref":"source:1","plane":"historical","score_micros":-9,"matched_terms":[]})).unwrap();
    assert_eq!(candidate.score_micros, -9);
    let cut = workflows::GenerationCut { kind: workflows::GenerationCutKind::Generation, value: 3 };
    assert_eq!(serde_json::to_value(cut).unwrap(), serde_json::json!({"kind":"generation","value":3}));
}

#[test]
fn case_work_contract_retains_bounded_intent_and_exact_observation_lineage() {
    let intent: workflows::ConversationIntent = serde_json::from_value(serde_json::json!({
        "executor_participant_id":"participant:model",
        "work_limits":{"invocations":2,"operations":1,"effects":0,"max_input_units":8192}
    })).unwrap();
    let serialized = serde_json::to_value(intent).unwrap();
    assert_eq!(serialized["work_limits"]["effects"], 0);
    assert!(serialized["work_limits"].get("max_output_tokens").is_none(),
        "historical Work requests keep their prior wire bytes");
    let bounded: workflows::ConversationIntent = serde_json::from_value(serde_json::json!({
        "executor_participant_id":"participant:model",
        "work_limits":{"invocations":2,"operations":1,"effects":0,
            "max_input_units":8192,"max_output_tokens":96}
    })).unwrap();
    assert_eq!(serde_json::to_value(bounded).unwrap()["work_limits"]["max_output_tokens"], 96);
    let observed: workflows::CaseWorkObservation = serde_json::from_value(serde_json::json!({
        "schema":"yai.case_work_observation.v1", "request_ref":"request:1",
        "participant_ref":"participant:model", "thread_ref":"thread:work",
        "observed_generation":7, "posture":"completed", "answer":"done",
        "steps":[{"ordinal":0,"source_ref":"source:0","selection_ref":"selection:0",
            "target_ref":"target:qualified","invocation_ref":"invocation:0",
            "provider_result_ref":"result:0","operation_ref":"operation:0",
            "outcome_refs":["observation:0"]}]
    })).unwrap();
    assert_eq!(observed.steps[0].target_ref, "target:qualified");
    assert_eq!(observed.steps[0].outcome_refs, ["observation:0"]);
    let catalog: serde_json::Value = serde_json::from_str(include_str!("../contract/operations.json")).unwrap();
    assert!(catalog["operations"].as_array().unwrap().iter().any(|operation|
        operation["operation_id"] == "conversation.work.resume"
            && operation["input_contract"] == "yai.conversation_work_resume_input.v1"));
}

#[test]
fn partial_and_unavailable_do_not_become_success_or_erase_evidence() {
    for mode in ["partial", "unavailable"] {
        let transport = mock(mode);
        let client = Client::discover(&transport, "discovery:1").unwrap();
        let response = client.execute("read:1", &ListCases {}).unwrap();
        if mode == "partial" {
            assert_eq!(response.state, ResultState::Partial);
            assert!(response.data.is_some());
        } else {
            assert_eq!(response.state, ResultState::CorePending);
            assert!(response.data.is_none());
            assert_eq!(response.raw.data.unwrap()["availability"], "producer_unavailable");
        }
        assert_eq!(transport.calls.borrow().len(), 2);
    }
}

#[test]
fn product_bootstrap_preserves_unknown_commercial_boundary() {
    use yai_sdk::workflows::{ProductBootstrapProjection, BootstrapPosture, EntitlementPosture};
    let value=serde_json::json!({
        "schema":"yai.product_bootstrap.v1","posture":"identity_required",
        "product":{"product":"yai","version":"0.1.0","access_policy":"not_enforced","license_verification":"unavailable"},
        "account":{"posture":"unavailable","linkage":null,"reason":"account_service_not_configured","sign_in_supported":false,"sign_out_supported":false},
        "profile":null,"local_home":"/tmp/qualified-profile",
        "principal":{"posture":"not_enrolled","principal_ref":null,"authentication_method":"posix"},
        "entitlement":{"posture":"missing","verification":"issuer_verifier_unavailable","access_granted":false,"observed_at_unix_ms":null,"claim":null},
        "workspaces":[],"selected_workspace_ref":null,"selected_workspace_available":false,
        "external_prerequisites":["product_account_service","trusted_entitlement_issuer_and_verifier"],
        "case_authority":"independent_current_yai_admission"
    });
    let parsed:ProductBootstrapProjection=serde_json::from_value(value.clone()).unwrap();
    assert_eq!(parsed.posture,BootstrapPosture::IdentityRequired);
    assert_eq!(parsed.entitlement.posture,EntitlementPosture::Missing);
    assert!(!parsed.entitlement.access_granted);assert!(parsed.profile.is_none());
    for posture in ["unverifiable","invalid","expired","not_yet_valid","version_not_covered","stale"] {
        let mut variant=value.clone();variant["entitlement"]["posture"]=serde_json::json!(posture);
        let parsed:ProductBootstrapProjection=serde_json::from_value(variant).unwrap();assert!(!parsed.entitlement.access_granted);
    }
    let mut broken=value;broken.as_object_mut().unwrap().remove("principal");
    assert!(serde_json::from_value::<ProductBootstrapProjection>(broken).is_err());
}
