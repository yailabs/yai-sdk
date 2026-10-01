//! Machine-readable exact SDK projection for cross-repository parity checks.
use yvex_sdk::{openai::PROFILE, MANAGEMENT_OPERATIONS, REQUEST_SCHEMA, RESPONSE_SCHEMA};

fn main() {
    println!(
        "{}",
        serde_json::json!({
            "schema": "platform.sdk.yvex.management.v1",
            "producer_request_schema": REQUEST_SCHEMA,
            "producer_response_schema": RESPONSE_SCHEMA,
            "operations": MANAGEMENT_OPERATIONS,
            "openai_capacity_profile": PROFILE,
            "finite_decision": {
                "client_schema": yvex_sdk::finite::CLIENT_SCHEMA,
                "producer_schema": yvex_sdk::finite::PRODUCER_SCHEMA,
                "native_client_compiled": yvex_sdk::finite::NATIVE_CLIENT_COMPILED,
                "posture": "client_contract_only_runtime_qualification_separate"
            },
            "posture": "contract_supported_runtime_not_implied"
        })
    );
}
