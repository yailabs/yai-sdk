//! A real read-only workflow. Requires an installed running Core and enrolled
//! local identity; this does not create a fixture or implicitly bootstrap access.
use yai_sdk::{client::{BoundLocalTransport, Client}, workflows::{CaseRefInput, EmptyInput}, ClientKind, ResultState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::args().nth(1).ok_or("usage: cases YAI_HOME")?;
    let transport = BoundLocalTransport::connect(home, ClientKind::External)?;
    let client = Client::discover(transport, "example:capabilities")?;
    let result = client.cases().list("example:list", &EmptyInput {})?;
    if result.state != ResultState::Success {
        println!("Core response: {:?}; {:?}", result.state, result.error);
        return Ok(());
    }
    for case in result.data.ok_or("missing successful projection")?.cases {
        let opened = client.cases().open("example:open", &CaseRefInput { case_ref: case.case_ref })?;
        println!("{:?}: {:?}", opened.state, opened.data);
    }
    Ok(())
}
