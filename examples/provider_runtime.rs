//! Read-only public provider administration, never direct inference or CLI parsing.
use std::io::Read;
use yai_sdk::{client::{BoundLocalTransport, Client, Response}, workflows::*, ClientKind, ResultState};

fn show<T>(response: Response<T>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(&response.raw)?);
    if response.state != ResultState::Success {
        return Err("Core observation refused; no mutation, SEND or retry performed".into());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() < 2 { return Err("usage: provider_runtime YAI_HOME protocols | readiness CASE AUTHOR | compare PUBLIC_COMPARISON_FILE".into()); }
    let client = Client::discover(BoundLocalTransport::connect(&args[0], ClientKind::External)?, "provider-example:discover")?;
    match (args[1].as_str(), args.len()) {
        ("protocols", 2) => show(client.provider().protocols("provider-example:protocols", &ProviderProtocolInventoryInput {})?)?,
        ("readiness", 4) => show(client.conversation().readiness("provider-example:readiness", &ConversationReadinessInput {
            case_ref: args[2].clone(), author_participant_ref: args[3].clone(),
        })?)?,
        ("compare", 3) => {
            let mut input = Vec::new();
            std::fs::File::open(&args[2])?.take(131073).read_to_end(&mut input)?;
            if input.len() > 131072 { return Err("public comparison exceeds 128 KiB bound".into()); }
            let input: ProviderLabCompareInput = serde_json::from_slice(&input)?;
            show(client.provider().compare_runs("provider-example:compare", &input)?)?;
        }
        _ => return Err("invalid read-only provider example arguments".into()),
    }
    Ok(())
}
