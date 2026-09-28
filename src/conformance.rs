//! Reusable contract assertions, not a mock Case engine.
use crate::{projections::ApplicationCatalog, *};
use std::collections::BTreeSet;

pub fn validate_catalog(result: &OperationResult) -> Result<ApplicationCatalog, String> {
    if result.operation_ref != "application.capabilities"
        || result.result_state != ResultState::Success
    {
        return Err("catalog_result_not_success".into());
    }
    let catalog: ApplicationCatalog =
        serde_json::from_value(result.data.clone().ok_or("catalog_missing")?)
            .map_err(|e| format!("catalog_projection_invalid:{e}"))?;
    if catalog.schema != CAPABILITY_CATALOG_SCHEMA
        || catalog.application_protocol != APPLICATION_PROTOCOL
    {
        return Err("catalog_identity_mismatch".into());
    }
    let mut operations = BTreeSet::new();
    for op in &catalog.operations {
        if op.operation_id.is_empty()
            || op.input_contract.is_empty()
            || op.output_contract.is_empty()
            || !operations.insert(op.operation_id.as_str())
        {
            return Err("catalog_operation_invalid".into());
        }
    }
    let mut capabilities = BTreeSet::new();
    for capability in &catalog.capabilities {
        if !capabilities.insert(capability.capability_id.as_str())
            || capability
                .application_operation_ids
                .iter()
                .any(|id| !operations.contains(id.as_str()))
        {
            return Err("catalog_capability_invalid".into());
        }
    }
    Ok(catalog)
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
