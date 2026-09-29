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
            "posture": "contract_supported_runtime_not_implied"
        })
    );
}
