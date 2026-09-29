//! Explicit real mutation example. Caller chooses Tenant, new Case and Participant.
//! Never run against an operator profile merely to exercise a test.
use yai_sdk::{client::{BoundLocalTransport, Client, Response}, workflows::*, ClientKind, ResultState};

fn require<T>(response: Response<T>) -> Result<T, Box<dyn std::error::Error>> {
    if response.state != ResultState::Success {
        return Err(format!("Core refused: {:?} {:?}", response.state, response.error).into());
    }
    response.data.ok_or_else(|| "missing successful projection".into())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 4 { return Err("usage: create_case YAI_HOME TENANT CASE PARTICIPANT".into()); }
    let client = Client::discover(BoundLocalTransport::connect(&args[0], ClientKind::External)?, "example:discover")?;
    let created = require(client.cases().create("example:create", &CaseCreateInput {
        tenant_id: args[1].clone(), case_ref: args[2].clone(),
    })?)?;
    println!("created: {} generation={}", created.state.case_id, created.state.generation);
    let role = require(client.authority().add_role("example:role", &ParticipantRoleInput {
        case_ref: args[2].clone(), participant_ref: args[3].clone(), role: "operator".into(),
    })?)?;
    println!("role receipt: {:?} generation={}", role.transition_ref, role.state.generation);
    let linked = require(client.authority().link_principal("example:link", &ParticipantLinkInput {
        case_ref: args[2].clone(), participant_ref: args[3].clone(), principal_ref: "self".into(),
    })?)?;
    println!("link receipt: {:?} generation={}", linked.transition_ref, linked.state.generation);
    let opened = require(client.cases().open("example:open", &CaseRefInput { case_ref: args[2].clone() })?)?;
    println!("opened: {} generation={} participant={}", opened.case_ref, opened.generation, opened.participant_ref);
    Ok(())
}
