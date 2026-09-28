use crate::*;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Read, Write};
pub const MAX_FRAME_BYTES: usize = 8 * 1024 * 1024;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ClientFrame {
    Handshake {
        protocol: String,
        sdk_version: String,
        client_id: String,
        client_kind: ClientKind,
        pid: u32,
        yai_home_identity: String,
    },
    ApplicationRequest {
        request: OperationRequest,
    },
    Subscribe,
    HostControl {
        action: HostControl,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HostControl {
    Status,
    Shutdown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ServerFrame {
    Handshake {
        protocol: String,
        compatibility: CompatibilityIdentity,
        host_instance_id: String,
        yai_home_identity: String,
    },
    ApplicationResponse {
        result: OperationResult,
    },
    Subscribed {
        host_instance_id: String,
    },
    Event {
        event: CaseUpdate,
    },
    Heartbeat {
        telemetry: HostTelemetry,
    },
    HostStatus {
        telemetry: HostTelemetry,
    },
    Shutdown {
        reason: String,
    },
    Error {
        code: String,
        message: String,
    },
}
pub fn write_frame<T: Serialize>(writer: &mut impl Write, frame: &T) -> Result<(), String> {
    let bytes = serde_json::to_vec(frame)
        .map_err(|error| format!("host_frame_serialize_failed:{error}"))?;
    // The newline delimiter is part of the bounded frame on both sides.
    if bytes.len() >= MAX_FRAME_BYTES {
        return Err("host_frame_too_large".into());
    }
    writer
        .write_all(&bytes)
        .map_err(|error| format!("host_frame_write_failed:{error}"))?;
    writer
        .write_all(b"\n")
        .map_err(|error| format!("host_frame_write_failed:{error}"))?;
    writer
        .flush()
        .map_err(|error| format!("host_frame_flush_failed:{error}"))
}

pub fn read_frame<T: for<'de> Deserialize<'de>>(reader: &mut impl BufRead) -> Result<T, String> {
    let mut bytes = Vec::new();
    let read = reader
        .take((MAX_FRAME_BYTES + 1) as u64)
        .read_until(b'\n', &mut bytes)
        .map_err(|error| format!("host_frame_read_failed:{error}"))?;
    if read == 0 {
        return Err("host_transport_closed".into());
    }
    if bytes.len() > MAX_FRAME_BYTES || !bytes.ends_with(b"\n") {
        return Err("host_frame_too_large".into());
    }
    serde_json::from_slice(&bytes).map_err(|error| format!("host_frame_invalid:{error}"))
}
