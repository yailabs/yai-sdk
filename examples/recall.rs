//! Read a caller-selected Case through current disclosure; never read Core stores.
use yai_sdk::{client::{BoundLocalTransport, Client}, workflows::*, ClientKind, ResultState};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 { return Err("usage: recall YAI_HOME CASE QUERY".into()); }
    let client = Client::discover(BoundLocalTransport::connect(&args[0], ClientKind::External)?, "recall:discover")?;
    let identity = client.identity().current("recall:identity", &EmptyInput {})?;
    if identity.state != ResultState::Success { return Err(format!("identity refused: {:?}", identity.raw).into()); }
    let opened = client.cases().open("recall:open", &CaseRefInput { case_ref: args[1].clone() })?;
    if opened.state != ResultState::Success { return Err(format!("open refused: {:?}", opened.raw).into()); }
    let case = opened.data.ok_or("missing Case attachment")?;
    let response = client.memory().recall("recall:query", &RecallInput { request: RecallQuery {
        schema: "yai.recall_request.v2".into(), case_id: case.case_ref,
        expected_generation: case.generation, participant_id: case.participant_ref,
        at: GenerationCut { kind: GenerationCutKind::Generation, value: case.generation },
        query: args[2].clone(), required_refs: vec![],
        bounds: RecallBounds { candidates:16, events:64, relations:128, segments:16,
            expansion_depth:4, semantic_units:16384, bytes:1048576 },
    }})?;
    println!("{}", serde_json::to_string_pretty(&response.raw)?);
    if response.state != ResultState::Success { return Err("Recall did not succeed; see exact refusal".into()); }
    Ok(())
}
