//! Reusable contract assertions, not a mock Case engine.
use crate::{projections::ApplicationCatalog, *};

/// Exercise real disconnect-after-write without implementing or predicting semantics.
/// Success proves a complete frame was written, never that Core admitted it.
pub fn disconnect_after_dispatch(
    client: HostClient,
    request: OperationRequest,
) -> Result<(), String> {
    client.disconnect_after_dispatch(request)
}

pub fn validate_catalog(result: &OperationResult) -> Result<ApplicationCatalog, String> {
    crate::validate_catalog_result(result)
}

/// Core-side release conformance, stricter than a client's capability discovery.
/// A client may use an older subset; a Core claiming this complete released
/// inventory must implement every published identity with its admitted posture.
pub fn validate_released_catalog(result: &OperationResult) -> Result<usize, String> {
    let actual = validate_catalog(result)?;
    let released: serde_json::Value = serde_json::from_str(include_str!("../contract/operations.json"))
        .map_err(|error| format!("released_catalog_invalid:{error}"))?;
    let operations = released["operations"].as_array().ok_or("released_operations_missing")?;
    for expected in operations {
        let id = expected["operation_id"].as_str().ok_or("released_operation_invalid")?;
        let observed = actual.operations.iter().find(|op| op.operation_id == id)
            .ok_or_else(|| format!("released_operation_missing:{id}"))?;
        for (field, value) in [("input_contract", &observed.input_contract),
            ("output_contract", &observed.output_contract), ("impact", &observed.impact),
            ("authority", &observed.authority)]
        {
            if expected[field].as_str() != Some(value.as_str()) {
                return Err(format!("released_operation_mismatch:{id}:{field}"));
            }
        }
    }
    Ok(operations.len())
}

pub fn validate_update(update: &CaseUpdate) -> Result<(), String> {
    if update.protocol != APPLICATION_PROTOCOL
        || update.case_ref.is_empty()
        || update.event_ref.is_empty()
        || update.event_type.is_empty()
        || update.cursor.is_empty()
    {
        return Err("case_update_invalid".into());
    }
    Ok(())
}
