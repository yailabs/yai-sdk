//! Structured conformance client; also useful from non-Rust test harnesses.
use yai_sdk::*;

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut home = None;
    let mut operation = None;
    let mut input = serde_json::json!({});
    let mut correlation = "sdk:conformance".to_owned();
    let mut status = false;
    let mut subscribe = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--home" => home = args.next(),
            "--operation" => operation = args.next(),
            "--inputs" => {
                input = serde_json::from_str(&args.next().ok_or("missing_inputs")?)
                    .map_err(|e| e.to_string())?
            }
            "--ref" => correlation = args.next().ok_or("missing_ref")?,
            "--status" => status = true,
            "--subscribe" => subscribe = true,
            _ => return Err(format!("unsupported_argument:{arg}")),
        }
    }
    let client = HostClient::connect(home.ok_or("missing_home")?, ClientKind::Qualification)?;
    if status {
        println!(
            "{}",
            serde_json::to_string(&client.status()?).map_err(|e| e.to_string())?
        );
    } else if subscribe {
        client.subscribe(|event| {
            let value = match event {
                HostEvent::Case(update) => serde_json::json!({"kind":"event", "event": update}),
                HostEvent::Heartbeat(telemetry) => {
                    serde_json::json!({"kind":"heartbeat", "telemetry":telemetry})
                }
                HostEvent::Shutdown(reason) => {
                    serde_json::json!({"kind":"shutdown", "reason":reason})
                }
            };
            println!("{value}");
            Ok(())
        })?;
    } else {
        let result = client
            .call_typed(OperationRequest {
                protocol: APPLICATION_PROTOCOL.into(),
                operation_ref: operation.ok_or("missing_operation")?,
                correlation_ref: correlation,
                input,
            })
            .map_err(|e| {
                format!(
                    "{}:outcome_indeterminate={}",
                    e.code, e.outcome_indeterminate
                )
            })?;
        println!(
            "{}",
            serde_json::to_string(&result).map_err(|e| e.to_string())?
        );
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
