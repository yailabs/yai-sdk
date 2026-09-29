//! Provider catalog observation through YAI. Never connect to YVEX directly.
use yai_sdk::{client::{BoundLocalTransport, Client}, workflows::RegisteredModelsInput, ClientKind, ResultState};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 { return Err("usage: models YAI_HOME TENANT TARGET".into()); }
    let client = Client::discover(BoundLocalTransport::connect(&args[0], ClientKind::External)?, "models:discover")?;
    let result = client.compute().models("models:read", &RegisteredModelsInput {
        tenant_id: args[1].clone(), target_ref: args[2].clone(),
    })?;
    println!("{}", serde_json::to_string_pretty(&result.raw)?);
    if result.state != ResultState::Success { return Err("catalog unavailable or refused; no inference attempted".into()); }
    Ok(())
}
