use std::io::BufReader;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::sync::atomic::{AtomicU64, Ordering};
use yai_sdk::wire::*;
use yai_sdk::*;

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn mock_host(
    mismatch: bool,
    lose_response: bool,
) -> (std::path::PathBuf, std::thread::JoinHandle<()>) {
    let home = std::env::temp_dir().join(format!(
        "yai-sdk-contract-{}-{}",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(home.join("run/host")).unwrap();
    let home = std::fs::canonicalize(home).unwrap();
    let endpoint = home.join("run/host/application.sock");
    let listener = UnixListener::bind(&endpoint).unwrap();
    std::fs::set_permissions(&endpoint, std::fs::Permissions::from_mode(0o600)).unwrap();
    let discovery = HostDiscovery {
        schema: HOST_DISCOVERY_SCHEMA.into(),
        protocol: HOST_PROTOCOL.into(),
        endpoint: endpoint.display().to_string(),
        pid: std::process::id(),
        process_identity: LocalProcessIdentity::capture(std::process::id()).unwrap(),
        instance_id: "mock:1".into(),
        started_at_unix_ms: 1,
        version: "0.1.9".into(),
        build: "mock-not-core".into(),
        yai_home: home.display().to_string(),
        yai_home_identity: platform::home_identity(&home),
    };
    let path = home.join("run/host/discovery.json");
    std::fs::write(&path, serde_json::to_vec(&discovery).unwrap()).unwrap();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).unwrap();
    let worker = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let frame: ClientFrame = read_frame(&mut reader).unwrap();
        assert!(
            matches!(frame, ClientFrame::Handshake { sdk_version, .. } if sdk_version == SDK_VERSION)
        );
        let mut compatibility = CompatibilityIdentity::current("0.1.9");
        if mismatch {
            compatibility.protocol = "yai.client.v99".into();
        }
        write_frame(
            &mut stream,
            &ServerFrame::Handshake {
                protocol: HOST_PROTOCOL.into(),
                compatibility,
                host_instance_id: discovery.instance_id,
                yai_home_identity: discovery.yai_home_identity,
            },
        )
        .unwrap();
        if mismatch {
            return;
        }
        let ClientFrame::ApplicationRequest { request } = read_frame(&mut reader).unwrap() else {
            panic!("request required")
        };
        if lose_response {
            return;
        }
        write_frame(
            &mut stream,
            &ServerFrame::ApplicationResponse {
                result: OperationResult {
                    operation_ref: if request.correlation_ref == "bad-operation" {
                        "foreign.operation".into()
                    } else {
                        request.operation_ref
                    },
                    correlation_ref: if request.correlation_ref == "bad-correlation" {
                        "foreign-id".into()
                    } else {
                        request.correlation_ref
                    },
                    result_state: ResultState::Success,
                    data: Some(serde_json::json!({"cases": [], "authority": "mock"})),
                    error: None,
                },
            },
        )
        .unwrap();
    });
    (home, worker)
}

fn request() -> OperationRequest {
    OperationRequest {
        protocol: APPLICATION_PROTOCOL.into(),
        operation_ref: "case.list".into(),
        correlation_ref: "contract:1".into(),
        input: serde_json::json!({}),
    }
}

#[test]
fn bound_transport_refuses_replacement_without_application_dispatch() {
    let home = std::env::temp_dir().join(format!("yai-sdk-replacement-{}", std::process::id()));
    std::fs::create_dir_all(home.join("run/host")).unwrap();
    let endpoint = home.join("run/host/application.sock");
    let listener = UnixListener::bind(&endpoint).unwrap();
    std::fs::set_permissions(&endpoint, std::fs::Permissions::from_mode(0o600)).unwrap();
    let mut discovery = HostDiscovery {
        schema: HOST_DISCOVERY_SCHEMA.into(), protocol: HOST_PROTOCOL.into(),
        endpoint: endpoint.display().to_string(), pid: std::process::id(),
        process_identity: LocalProcessIdentity::capture(std::process::id()).unwrap(),
        instance_id: "mock:original".into(), started_at_unix_ms:1,
        version:"0.1.0".into(), build:"fixture".into(), yai_home:home.display().to_string(),
        yai_home_identity:platform::home_identity(&home),
    };
    let path = home.join("run/host/discovery.json");
    std::fs::write(&path, serde_json::to_vec(&discovery).unwrap()).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    let home_identity = discovery.yai_home_identity.clone();
    let server = std::thread::spawn(move || {
        for instance in ["mock:original", "mock:replacement"] {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(std::time::Duration::from_secs(2))).unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            assert!(matches!(read_frame::<ClientFrame>(&mut reader).unwrap(), ClientFrame::Handshake { .. }));
            write_frame(&mut stream, &ServerFrame::Handshake { protocol: HOST_PROTOCOL.into(),
                compatibility:CompatibilityIdentity::current("0.1.0"), host_instance_id:instance.into(),
                yai_home_identity:home_identity.clone() }).unwrap();
            assert!(read_frame::<ClientFrame>(&mut reader).is_err(), "no application dispatch allowed");
        }
    });
    let transport = client::BoundLocalTransport::connect(&home, ClientKind::Qualification).unwrap();
    discovery.instance_id = "mock:replacement".into();
    std::fs::write(&path, serde_json::to_vec(&discovery).unwrap()).unwrap();
    let error = transport.execute(request()).unwrap_err();
    assert_eq!(error.code, "host_replaced_rediscovery_required");
    assert!(!error.outcome_indeterminate);
    server.join().unwrap();
    std::fs::remove_dir_all(home).unwrap();
}

#[test]
fn independent_client_roundtrip_preserves_identity_and_projection() {
    let (home, worker) = mock_host(false, false);
    let result = HostClient::connect(&home, ClientKind::Qualification)
        .unwrap()
        .call_typed(request())
        .unwrap();
    assert_eq!(result.correlation_ref, "contract:1");
    assert_eq!(result.operation_ref, "case.list");
    let projection: projections::CaseListProjection =
        serde_json::from_value(result.data.unwrap()).unwrap();
    assert!(projection.cases.is_empty());
    worker.join().unwrap();
    std::fs::remove_dir_all(home).unwrap();
}

#[test]
fn compatibility_refuses_before_ordinary_dispatch() {
    let (home, worker) = mock_host(true, false);
    assert!(
        matches!(HostClient::connect(&home, ClientKind::Qualification), Err(e) if e == "host_compatibility_mismatch")
    );
    worker.join().unwrap();
    std::fs::remove_dir_all(home).unwrap();
}

#[test]
fn lost_response_is_indeterminate_without_replay() {
    let (home, worker) = mock_host(false, true);
    let error = HostClient::connect(&home, ClientKind::Qualification)
        .unwrap()
        .call_typed(request())
        .unwrap_err();
    assert!(error.outcome_indeterminate);
    worker.join().unwrap();
    std::fs::remove_dir_all(home).unwrap();
}

#[test]
fn qualification_disconnect_writes_complete_request_without_observing_outcome() {
    let (home, worker) = mock_host(false, true);
    let client = HostClient::connect(&home, ClientKind::Qualification).unwrap();
    conformance::disconnect_after_dispatch(client, request()).unwrap();
    // Mock owner requires a complete ApplicationRequest before exiting.
    worker.join().unwrap();
    std::fs::remove_dir_all(home).unwrap();
}

#[test]
fn version_compatibility_is_not_git_or_patch_equality() {
    assert!(CompatibilityIdentity::current("0.1.99").validate().is_ok());
    assert!(CompatibilityIdentity::current("0.2.0").validate().is_ok());
    assert!(CompatibilityIdentity::current("0.1.0-rc.1")
        .validate()
        .is_ok());
    assert!(CompatibilityIdentity::current("not-a-version")
        .validate()
        .is_err());
}

#[test]
fn bounded_wire_rejects_truncated_and_invalid_frames() {
    assert!(read_frame::<ClientFrame>(&mut &b"{}"[..]).is_err());
    assert!(read_frame::<ClientFrame>(&mut &b"{}\n"[..]).is_err());
    assert!(read_frame::<ClientFrame>(&mut &vec![b'x'; MAX_FRAME_BYTES + 1][..]).is_err());
}

#[test]
fn wire_bound_includes_delimiter_symmetrically() {
    let accepted = serde_json::Value::String("x".repeat(MAX_FRAME_BYTES - 3));
    let mut bytes = Vec::new();
    write_frame(&mut bytes, &accepted).unwrap();
    assert_eq!(bytes.len(), MAX_FRAME_BYTES);
    assert_eq!(
        read_frame::<serde_json::Value>(&mut bytes.as_slice()).unwrap(),
        accepted
    );
    let refused = serde_json::Value::String("x".repeat(MAX_FRAME_BYTES - 2));
    assert_eq!(
        write_frame(&mut Vec::new(), &refused).unwrap_err(),
        "host_frame_too_large"
    );
}

#[test]
fn invalidation_contract_refuses_foreign_protocol_or_missing_identity() {
    let mut update = CaseUpdate {
        protocol: APPLICATION_PROTOCOL.into(),
        event_ref: "case-update:c:1".into(),
        event_type: "case_projection_invalidated".into(),
        event_family: "case".into(),
        case_ref: "c".into(),
        generation: 1,
        sequence: 1,
        cursor: "c:1:1".into(),
        affected_views: vec!["overview".into()],
    };
    assert!(conformance::validate_update(&update).is_ok());
    update.protocol = "foreign".into();
    assert!(conformance::validate_update(&update).is_err());
    update.protocol = APPLICATION_PROTOCOL.into();
    update.case_ref.clear();
    assert!(conformance::validate_update(&update).is_err());
}

#[test]
fn application_refusals_are_not_transport_failures() {
    for state in [
        ResultState::Unauthorized,
        ResultState::Stale,
        ResultState::CorePending,
        ResultState::NotImplemented,
    ] {
        let result = OperationResult {
            operation_ref: "operation".into(),
            correlation_ref: "id".into(),
            result_state: state,
            data: None,
            error: None,
        };
        let decoded: OperationResult =
            serde_json::from_slice(&serde_json::to_vec(&result).unwrap()).unwrap();
        assert_eq!(decoded.result_state, state);
    }
}

#[test]
fn active_process_identity_is_exact_and_stale_fails() {
    let mut identity = LocalProcessIdentity::capture(std::process::id()).unwrap();
    assert!(identity.is_live());
    identity.start_ticks += 1;
    assert!(!identity.is_live());
}

#[test]
fn response_operation_and_correlation_are_fenced() {
    for fault in ["bad-operation", "bad-correlation"] {
        let (home, worker) = mock_host(false, false);
        let mut request = request();
        request.correlation_ref = fault.into();
        let error = HostClient::connect(&home, ClientKind::Qualification)
            .unwrap()
            .call_typed(request)
            .unwrap_err();
        assert_eq!(error.code, "host_application_response_invalid");
        assert!(error.outcome_indeterminate);
        worker.join().unwrap();
        std::fs::remove_dir_all(home).unwrap();
    }
}

#[test]
fn discovery_security_and_lineage_fail_before_dispatch() {
    let home = std::env::temp_dir().join(format!("yai-sdk-security-{}", std::process::id()));
    std::fs::create_dir_all(home.join("run/host")).unwrap();
    let endpoint = home.join("run/host/application.sock");
    let _listener = UnixListener::bind(&endpoint).unwrap();
    std::fs::set_permissions(&endpoint, std::fs::Permissions::from_mode(0o600)).unwrap();
    let mut discovery = HostDiscovery {
        schema: HOST_DISCOVERY_SCHEMA.into(),
        protocol: HOST_PROTOCOL.into(),
        endpoint: endpoint.display().to_string(),
        pid: std::process::id(),
        process_identity: LocalProcessIdentity::capture(std::process::id()).unwrap(),
        instance_id: "security:1".into(),
        started_at_unix_ms: 1,
        version: "0.1.0".into(),
        build: "test".into(),
        yai_home: home.display().to_string(),
        yai_home_identity: platform::home_identity(&home),
    };
    let path = home.join("run/host/discovery.json");
    std::fs::write(&path, serde_json::to_vec(&discovery).unwrap()).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert!(
        matches!(HostClient::connect(&home, ClientKind::Qualification), Err(e) if e == "host_discovery_security_invalid")
    );
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    discovery.process_identity.start_ticks += 1;
    std::fs::write(&path, serde_json::to_vec(&discovery).unwrap()).unwrap();
    assert!(
        matches!(HostClient::connect(&home, ClientKind::Qualification), Err(e) if e == "host_discovery_stale")
    );
    discovery.process_identity =
        LocalProcessIdentity::capture(unsafe { libc::getppid() } as u32).unwrap();
    discovery.pid = discovery.process_identity.pid;
    std::fs::write(&path, serde_json::to_vec(&discovery).unwrap()).unwrap();
    assert!(
        matches!(HostClient::connect(&home, ClientKind::Qualification), Err(e) if e == "host_peer_process_mismatch")
    );
    std::fs::remove_dir_all(home).unwrap();
}
