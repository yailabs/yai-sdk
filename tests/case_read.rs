// Public wire conformance; not real inference or operator acceptance.
use yai_sdk::workflows::{CaseReadRequest, CaseReadResult, CaseOperation};

#[test]
fn case_read_payload_is_closed_and_variant_specific() {
    let workflow: CaseReadRequest = serde_json::from_value(serde_json::json!({"read":"workflow_inspect"})).unwrap();
    assert!(matches!(workflow, CaseReadRequest::WorkflowInspect {}));
    for value in [serde_json::json!({"read":"workflow_inspect","query":"ignored"}),
                  serde_json::json!({"read":"knowledge_search","query":"private","limit":4,"case_id":"case:foreign"}),
                  serde_json::json!({"read":"recall","query":"missing mandatory limit"})] {
        assert!(serde_json::from_value::<CaseReadRequest>(value).is_err());
    }
    assert!(serde_json::from_value::<CaseReadResult>(serde_json::json!({"kind":"recall","view":{}})).is_err());
}

#[test]
fn exact_operation_is_a_proposal_with_typed_origin() {
    let value = serde_json::json!({"schema":"yai.operation.v1","operation_id":"operation:proposed","operation_digest":"sha256:exact",
      "case_id":"case:isolated","participant_id":"participant:model","scope":{"case_id":"case:isolated","participant_refs":["participant:model"],"resource_refs":["resource:file"],"policy_refs":[]},
      "kind":"filesystem_write","resource_attachment_id":"resource:file","filesystem_write":{"relative_path":"config.toml","content":"checkpoint = 1","content_digest":"sha256:content","content_bytes":14},
      "origin":{"kind":"provider_result","provider_result_id":"provider-result:proposal","provider_invocation_id":"provider-invocation:exact"},"expected_case_generation":4});
    let operation: CaseOperation=serde_json::from_value(value).unwrap();
    let roundtrip=serde_json::to_value(&operation).unwrap();
    assert_eq!(roundtrip["kind"],"filesystem_write");
    assert!(roundtrip.get("applied").is_none());
    assert_eq!(roundtrip["origin"]["provider_result_id"],"provider-result:proposal");
}
