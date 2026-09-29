//! Verify a running Core's complete released operation inventory, read-only.
use yai_sdk::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::args().nth(1).ok_or("usage: conformance YAI_HOME")?;
    let response = HostClient::connect(home, ClientKind::Qualification)
        .map_err(|error| std::io::Error::other(error))?
        .call_typed(OperationRequest { protocol: APPLICATION_PROTOCOL.into(),
            operation_ref: "application.capabilities".into(), correlation_ref: "conformance:catalog".into(),
            input: serde_json::json!({}) })?;
    let count = conformance::validate_released_catalog(&response)
        .map_err(|error| std::io::Error::other(error))?;
    println!("PASS: {count} published operation identities match Core; semantic tests remain separate");
    Ok(())
}
