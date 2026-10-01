use crate::wire::*;
use crate::*;
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::BufReader;
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
const START_TIMEOUT: Duration = Duration::from_secs(8);
static CLIENT_COUNTER: AtomicU64 = AtomicU64::new(1);
impl HostTelemetry {
    fn stopped(home: &Path) -> Result<Self, String> {
        let canonical = canonical_home(home)?;
        Ok(Self {
            schema: HOST_TELEMETRY_SCHEMA.into(),
            state: "stopped".into(),
            pid: None,
            process_identity: None,
            instance_id: None,
            started_at_unix_ms: None,
            uptime_ms: 0,
            protocol: HOST_PROTOCOL.into(),
            version: "unknown".into(),
            build: "unknown".into(),
            executable_posture: None,
            installed_executable_posture: None,
            yai_home: canonical.display().to_string(),
            yai_home_identity: home_identity(&canonical),
            transport: "unix_domain_socket".into(),
            endpoint: None,
            endpoint_posture: "absent".into(),
            application_readiness: "host_absent".into(),
            connected_clients: 0,
            client_kinds: BTreeMap::new(),
            clients: Vec::new(),
            event_sequence: 0,
            last_activity_unix_ms: now_ms(),
            runtime_supervision: "not_integrated".into(),
            runtime_observation: None,
        })
    }
}

pub struct HostClient {
    #[cfg(unix)]
    stream: UnixStream,
    #[cfg(unix)]
    reader: BufReader<UnixStream>,
    pub discovery: HostDiscovery,
}

impl HostClient {
    /// Qualification hook: no response is observed and no outcome is implied.
    #[cfg(unix)]
    pub(crate) fn disconnect_after_dispatch(
        mut self,
        request: OperationRequest,
    ) -> Result<(), String> {
        write_frame(
            &mut self.stream,
            &ClientFrame::ApplicationRequest { request },
        )
    }
    #[cfg(unix)]
    pub fn connect(home: impl AsRef<Path>, kind: ClientKind) -> Result<Self, String> {
        let home = canonical_home(home.as_ref())?;
        let discovery = read_discovery(&home)?;
        validate_discovery(&home, &discovery)?;
        let stream = UnixStream::connect(&discovery.endpoint)
            .map_err(|error| format!("host_transport_unavailable:{error}"))?;
        verify_peer_same_user(&stream)?;
        if peer_credentials(&stream)?.pid != discovery.pid || !discovery.process_identity.is_live()
        {
            return Err("host_peer_process_mismatch".into());
        }
        let reader_stream = stream
            .try_clone()
            .map_err(|error| format!("host_transport_clone_failed:{error}"))?;
        let mut client = Self {
            stream,
            reader: BufReader::new(reader_stream),
            discovery,
        };
        let client_id = next_client_id(kind.label());
        let frame = ClientFrame::Handshake {
            protocol: HOST_PROTOCOL.into(),
            sdk_version: SDK_VERSION.into(),
            client_id,
            client_kind: kind,
            pid: std::process::id(),
            yai_home_identity: home_identity(&home),
        };
        write_frame(&mut client.stream, &frame)?;
        match read_frame::<ServerFrame>(&mut client.reader)? {
            ServerFrame::Handshake {
                protocol,
                compatibility,
                host_instance_id,
                yai_home_identity,
            } if protocol == HOST_PROTOCOL
                && host_instance_id == client.discovery.instance_id
                && yai_home_identity == client.discovery.yai_home_identity =>
            {
                compatibility.validate()?;
                if compatibility.core_version != client.discovery.version
                    || compatibility.protocol != client.discovery.protocol
                {
                    return Err("host_discovery_compatibility_mismatch".into());
                }
                Ok(client)
            }
            ServerFrame::Error { code, message } => Err(format!("{code}:{message}")),
            _ => Err("host_handshake_invalid".into()),
        }
    }

    #[cfg(not(unix))]
    pub fn connect(_home: impl AsRef<Path>, _kind: ClientKind) -> Result<Self, String> {
        Err("host_platform_unqualified".into())
    }

    /// No automatic retry: transport failure after dispatch may have admitted work.
    #[cfg(unix)]
    pub fn call_typed(
        self,
        request: OperationRequest,
    ) -> Result<OperationResult, crate::ClientError> {
        let mut checked = Vec::new();
        write_frame(
            &mut checked,
            &ClientFrame::ApplicationRequest {
                request: request.clone(),
            },
        )
        .map_err(|code| crate::ClientError {
            code,
            outcome_indeterminate: false,
        })?;
        self.call(request)
            .map_err(crate::ClientError::after_dispatch)
    }

    #[cfg(unix)]
    pub fn call(mut self, request: OperationRequest) -> Result<OperationResult, String> {
        let operation = request.operation_ref.clone();
        let correlation = request.correlation_ref.clone();
        write_frame(
            &mut self.stream,
            &ClientFrame::ApplicationRequest { request },
        )?;
        match read_frame::<ServerFrame>(&mut self.reader)? {
            ServerFrame::ApplicationResponse { result }
                if result.operation_ref == operation && result.correlation_ref == correlation =>
            {
                Ok(result)
            }
            ServerFrame::Error { code, message } => Err(format!("{code}:{message}")),
            _ => Err("host_application_response_invalid".into()),
        }
    }

    #[cfg(unix)]
    pub fn status(mut self) -> Result<HostTelemetry, String> {
        write_frame(
            &mut self.stream,
            &ClientFrame::HostControl {
                action: HostControl::Status,
            },
        )?;
        match read_frame::<ServerFrame>(&mut self.reader)? {
            ServerFrame::HostStatus { mut telemetry } => {
                annotate_executable_posture(&mut telemetry, &self.discovery);
                Ok(telemetry)
            }
            ServerFrame::Error { code, message } => Err(format!("{code}:{message}")),
            _ => Err("host_status_response_invalid".into()),
        }
    }

    #[cfg(unix)]
    pub fn shutdown(mut self) -> Result<HostTelemetry, String> {
        write_frame(
            &mut self.stream,
            &ClientFrame::HostControl {
                action: HostControl::Shutdown,
            },
        )?;
        match read_frame::<ServerFrame>(&mut self.reader)? {
            ServerFrame::HostStatus { telemetry } => Ok(telemetry),
            ServerFrame::Error { code, message } => Err(format!("{code}:{message}")),
            _ => Err("host_shutdown_response_invalid".into()),
        }
    }

    #[cfg(unix)]
    pub fn subscribe<F>(mut self, mut event: F) -> Result<(), String>
    where
        F: FnMut(HostEvent) -> Result<(), String>,
    {
        write_frame(&mut self.stream, &ClientFrame::Subscribe)?;
        match read_frame::<ServerFrame>(&mut self.reader)? {
            ServerFrame::Subscribed { host_instance_id }
                if host_instance_id == self.discovery.instance_id => {}
            ServerFrame::Error { code, message } => return Err(format!("{code}:{message}")),
            _ => return Err("host_subscription_response_invalid".into()),
        }
        loop {
            match read_frame::<ServerFrame>(&mut self.reader)? {
                ServerFrame::Event { event: update } => {
                    crate::conformance::validate_update(&update)?;
                    event(HostEvent::Case(update))?
                }
                ServerFrame::Heartbeat { mut telemetry } => {
                    annotate_executable_posture(&mut telemetry, &self.discovery);
                    event(HostEvent::Heartbeat(telemetry))?
                }
                ServerFrame::Shutdown { reason } => {
                    event(HostEvent::Shutdown(reason))?;
                    return Ok(());
                }
                ServerFrame::Error { code, message } => return Err(format!("{code}:{message}")),
                _ => return Err("host_subscription_frame_invalid".into()),
            }
        }
    }
}

pub fn observe(home: impl AsRef<Path>) -> Result<HostTelemetry, String> {
    match HostClient::connect(home.as_ref(), ClientKind::Cli) {
        Ok(client) => client.status(),
        Err(error) if is_absent_error(&error) => HostTelemetry::stopped(home.as_ref()),
        Err(error) => Err(error),
    }
}

pub fn start(
    home: impl AsRef<Path>,
    executable: impl AsRef<Path>,
    serve_arguments: &[&str],
) -> Result<HostTelemetry, String> {
    let home = canonical_home(home.as_ref())?;
    match HostClient::connect(&home, ClientKind::Cli).and_then(HostClient::status) {
        Ok(status) => return Ok(status),
        Err(error) if is_absent_error(&error) => {}
        Err(error) => return Err(error),
    }
    let log_path = HostPaths::prepare(&home)?.log;
    let stdout = OpenOptions::new()
        .create(true)
        .append(true)
        .mode(0o600)
        .open(&log_path)
        .map_err(|error| format!("host_log_open_failed:{error}"))?;
    let stderr = stdout
        .try_clone()
        .map_err(|error| format!("host_log_clone_failed:{error}"))?;
    let mut command = Command::new(executable.as_ref());
    command
        .args(serve_arguments)
        .env("YAI_HOME", &home)
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));
    #[cfg(unix)]
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let mut child = command
        .spawn()
        .map_err(|error| format!("host_spawn_failed:{error}"))?;
    let deadline = std::time::Instant::now() + START_TIMEOUT;
    let mut child_exit = None;
    loop {
        if let Ok(status) = HostClient::connect(&home, ClientKind::Cli).and_then(HostClient::status)
        {
            return Ok(status);
        }
        if child_exit.is_none() {
            child_exit = child
                .try_wait()
                .map_err(|error| format!("host_spawn_wait_failed:{error}"))?;
        }
        if std::time::Instant::now() >= deadline {
            return Err(child_exit
                .map(|status| format!("host_start_failed:exit={status}"))
                .unwrap_or_else(|| "host_start_timeout".into()));
        }
        thread::sleep(Duration::from_millis(50));
    }
}

pub fn stop(home: impl AsRef<Path>) -> Result<HostTelemetry, String> {
    let home = canonical_home(home.as_ref())?;
    let prior = match HostClient::connect(&home, ClientKind::Cli) {
        Ok(client) => client.shutdown()?,
        Err(error) if is_absent_error(&error) => return HostTelemetry::stopped(&home),
        Err(error) => return Err(error),
    };
    let deadline = std::time::Instant::now() + START_TIMEOUT;
    let paths = HostPaths::prepare(&home)?;
    while std::time::Instant::now() < deadline {
        if HostClient::connect(&home, ClientKind::Cli).is_err()
            && !paths.discovery.exists()
            && !paths.socket.exists()
        {
            return HostTelemetry::stopped(&home);
        }
        thread::sleep(Duration::from_millis(50));
    }
    Err(format!(
        "host_stop_timeout:instance={}",
        prior.instance_id.unwrap_or_default()
    ))
}

pub fn restart(
    home: impl AsRef<Path>,
    executable: impl AsRef<Path>,
    serve_arguments: &[&str],
) -> Result<HostTelemetry, String> {
    let home = canonical_home(home.as_ref())?;
    let _ = stop(&home)?;
    start(&home, executable, serve_arguments)
}

struct HostPaths {
    socket: PathBuf,
    discovery: PathBuf,
    log: PathBuf,
}

impl HostPaths {
    fn prepare(home: &Path) -> Result<Self, String> {
        let run = home.join("run").join("host");
        let log_root = home.join("log");
        fs::create_dir_all(&run).map_err(|error| format!("host_run_root_failed:{error}"))?;
        fs::create_dir_all(&log_root).map_err(|error| format!("host_log_root_failed:{error}"))?;
        #[cfg(unix)]
        {
            fs::set_permissions(&run, fs::Permissions::from_mode(0o700))
                .map_err(|error| format!("host_run_permissions_failed:{error}"))?;
            fs::set_permissions(&log_root, fs::Permissions::from_mode(0o700))
                .map_err(|error| format!("host_log_permissions_failed:{error}"))?;
        }
        Ok(Self {
            socket: run.join("application.sock"),
            discovery: run.join("discovery.json"),
            log: log_root.join("yai-host.log"),
        })
    }
}

pub fn read_discovery(home: &Path) -> Result<HostDiscovery, String> {
    let path = home.join("run/host/discovery.json");
    let bytes = fs::read(&path).map_err(|error| format!("host_discovery_unavailable:{error}"))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("host_discovery_invalid:{error}"))
}

pub fn validate_discovery(home: &Path, discovery: &HostDiscovery) -> Result<(), String> {
    #[cfg(unix)]
    {
        let metadata = fs::symlink_metadata(home.join("run/host/discovery.json"))
            .map_err(|error| format!("host_discovery_unavailable:{error}"))?;
        if !metadata.file_type().is_file()
            || metadata.uid() != unsafe { libc::geteuid() }
            || metadata.permissions().mode() & 0o077 != 0
        {
            return Err("host_discovery_security_invalid".into());
        }
    }
    if discovery.yai_home != home.display().to_string()
        || discovery.yai_home_identity != home_identity(home)
    {
        return Err("host_discovery_yai_home_mismatch".into());
    }
    if !discovery.process_identity.is_live() {
        return Err("host_discovery_stale".into());
    }
    if discovery.pid != discovery.process_identity.pid {
        return Err("host_discovery_process_mismatch".into());
    }
    if discovery.schema != HOST_DISCOVERY_SCHEMA || discovery.protocol != HOST_PROTOCOL {
        return Err("host_discovery_protocol_mismatch".into());
    }
    let endpoint = Path::new(&discovery.endpoint);
    if endpoint != home.join("run/host/application.sock") {
        return Err("host_discovery_endpoint_mismatch".into());
    }
    #[cfg(unix)]
    {
        let metadata = fs::symlink_metadata(endpoint)
            .map_err(|error| format!("host_endpoint_unavailable:{error}"))?;
        if !metadata.file_type().is_socket()
            || metadata.uid() != unsafe { libc::geteuid() }
            || metadata.permissions().mode() & 0o077 != 0
        {
            return Err("host_endpoint_security_invalid".into());
        }
    }
    Ok(())
}

#[cfg(unix)]
pub struct PeerCredentials {
    pub pid: u32,
    pub uid: u32,
}

#[cfg(target_os = "linux")]
pub fn peer_credentials(stream: &UnixStream) -> Result<PeerCredentials, String> {
    let mut credentials = libc::ucred {
        pid: 0,
        uid: 0,
        gid: 0,
    };
    let mut length = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    let result = unsafe {
        libc::getsockopt(
            stream.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            &mut credentials as *mut _ as *mut libc::c_void,
            &mut length,
        )
    };
    if result != 0 {
        return Err(format!(
            "host_peer_credentials_failed:{}",
            std::io::Error::last_os_error()
        ));
    }
    if length as usize != std::mem::size_of::<libc::ucred>() || credentials.pid <= 0 {
        return Err("host_peer_credentials_invalid".into());
    }
    Ok(PeerCredentials {
        pid: credentials.pid as u32,
        uid: credentials.uid,
    })
}

#[cfg(target_os = "macos")]
pub fn peer_credentials(stream: &UnixStream) -> Result<PeerCredentials, String> {
    // Darwin reports effective credentials and the peer PID through separate native queries.
    // Both must succeed; the caller still authenticates the advertised process/start identity.
    let mut uid: libc::uid_t = 0;
    let mut gid: libc::gid_t = 0;
    let mut pid: libc::pid_t = 0;
    let mut length = std::mem::size_of::<libc::pid_t>() as libc::socklen_t;
    if unsafe { libc::getpeereid(stream.as_raw_fd(), &mut uid, &mut gid) } != 0 {
        return Err(format!(
            "host_peer_credentials_failed:{}",
            std::io::Error::last_os_error()
        ));
    }
    let result = unsafe {
        libc::getsockopt(
            stream.as_raw_fd(),
            libc::SOL_LOCAL,
            libc::LOCAL_PEERPID,
            &mut pid as *mut _ as *mut libc::c_void,
            &mut length,
        )
    };
    if result != 0 {
        return Err(format!(
            "host_peer_credentials_failed:{}",
            std::io::Error::last_os_error()
        ));
    }
    if length as usize != std::mem::size_of::<libc::pid_t>() || pid <= 0 {
        return Err("host_peer_credentials_invalid".into());
    }
    Ok(PeerCredentials { pid: pid as u32, uid })
}

#[cfg(all(unix, not(any(target_os = "linux", target_os = "macos"))))]
pub fn peer_credentials(_stream: &UnixStream) -> Result<PeerCredentials, String> {
    Err("host_peer_credentials_unsupported_platform".into())
}

#[cfg(unix)]
pub fn verify_peer_same_user(stream: &UnixStream) -> Result<(), String> {
    let peer = peer_credentials(stream)?;
    if peer.uid != unsafe { libc::geteuid() } {
        return Err("host_peer_uid_mismatch".into());
    }
    Ok(())
}

pub fn canonical_home(home: &Path) -> Result<PathBuf, String> {
    if !home.exists() {
        fs::create_dir_all(home).map_err(|error| format!("yai_home_create_failed:{error}"))?;
    }
    let canonical = fs::canonicalize(home).map_err(|error| format!("yai_home_invalid:{error}"))?;
    #[cfg(unix)]
    {
        let metadata = fs::metadata(&canonical)
            .map_err(|error| format!("yai_home_metadata_failed:{error}"))?;
        if metadata.uid() != unsafe { libc::geteuid() } {
            return Err("yai_home_not_owned_by_current_user".into());
        }
    }
    Ok(canonical)
}

pub fn home_identity(home: &Path) -> String {
    format!("yai-home:{}", stable_digest(&home.display().to_string()))
}

fn next_client_id(kind: &str) -> String {
    let sequence = CLIENT_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{kind}:{}:{sequence}", std::process::id())
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

fn annotate_executable_posture(telemetry: &mut HostTelemetry, discovery: &HostDiscovery) {
    if telemetry.state != "running" {
        return;
    }
    // The local client can inspect an older Host that predates this optional
    // telemetry field. Fence the PID with the discovery process identity first.
    telemetry.executable_posture = Some(
        if telemetry.pid == Some(discovery.pid)
            && telemetry.process_identity.as_deref()
                == Some(discovery.process_identity.canonical_identity().as_str())
        {
            executable_link_posture(&discovery.process_identity)
        } else {
            "unknown"
        }
        .into(),
    );
    if let Some(selected) = std::env::var_os("YAI_EXECUTABLE") {
        telemetry.installed_executable_posture = Some(
            if telemetry.pid == Some(discovery.pid)
                && telemetry.process_identity.as_deref()
                    == Some(discovery.process_identity.canonical_identity().as_str())
            {
                installed_executable_posture(&discovery.process_identity, Path::new(&selected))
            } else {
                "unknown"
            }
            .into(),
        );
    }
}

fn installed_executable_posture(identity: &LocalProcessIdentity, installed: &Path) -> &'static str {
    #[cfg(target_os = "linux")]
    {
        if !identity.is_live() {
            return "unknown";
        }
        let running = fs::metadata(format!("/proc/{}/exe", identity.pid));
        let selected = fs::metadata(installed);
        if !identity.is_live() {
            return "unknown";
        }
        match (running, selected) {
            (_, Err(error)) if error.kind() == std::io::ErrorKind::NotFound => "installed_missing",
            (Ok(running), Ok(selected)) if running.dev() == selected.dev()
                && running.ino() == selected.ino() => "matches_running",
            (Ok(_), Ok(_)) => "different_from_running",
            _ => "unknown",
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (identity, installed);
        "unknown"
    }
}

fn executable_link_posture(identity: &LocalProcessIdentity) -> &'static str {
    #[cfg(target_os = "linux")]
    {
        if !identity.is_live() {
            return "unknown";
        }
        let target = fs::read_link(format!("/proc/{}/exe", identity.pid));
        if !identity.is_live() {
            return "unknown";
        }
        match target {
            Ok(target) if target.as_os_str().as_bytes().ends_with(b" (deleted)") => {
                "replaced_on_disk"
            }
            Ok(_) => "linked",
            Err(_) => "unknown",
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = identity;
        "unknown"
    }
}

fn is_absent_error(error: &str) -> bool {
    error.starts_with("host_discovery_unavailable")
        || error.starts_with("host_endpoint_unavailable")
        || error.starts_with("host_transport_unavailable")
        || error == "host_discovery_stale"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn absent_core_never_inherits_sdk_product_identity() {
        let home = std::env::temp_dir().join(format!(
            "yai-sdk-absent-{}-{}",
            std::process::id(),
            now_ms()
        ));
        let telemetry = observe(&home).unwrap();
        assert_eq!(telemetry.state, "stopped");
        assert_eq!(telemetry.version, "unknown");
        assert_eq!(telemetry.build, "unknown");
        fs::remove_dir_all(home).unwrap();
    }
    #[test]
    fn exact_process_executable_replacement_is_observed_without_pid_reuse_guessing() {
        let home = std::env::temp_dir().join(format!(
            "yai-sdk-executable-{}-{}",
            std::process::id(),
            now_ms()
        ));
        fs::create_dir(&home).unwrap();
        let executable = home.join("sleep-copy");
        fs::copy("/usr/bin/sleep", &executable).unwrap();
        let mut child = Command::new(&executable).arg("30").spawn().unwrap();
        let identity = LocalProcessIdentity::capture(child.id()).unwrap();
        assert_eq!(executable_link_posture(&identity), "linked");
        assert_eq!(installed_executable_posture(&identity, &executable), "matches_running");
        let replacement = home.join("replacement");
        fs::copy("/usr/bin/sleep", &replacement).unwrap();
        assert_eq!(installed_executable_posture(&identity, &replacement), "different_from_running");
        fs::remove_file(&executable).unwrap();
        assert_eq!(executable_link_posture(&identity), "replaced_on_disk");
        assert_eq!(installed_executable_posture(&identity, &executable), "installed_missing");
        let mut wrong_identity = identity.clone();
        wrong_identity.start_ticks += 1;
        assert_eq!(executable_link_posture(&wrong_identity), "unknown");
        assert_eq!(installed_executable_posture(&wrong_identity, &replacement), "unknown");
        child.kill().unwrap();
        child.wait().unwrap();
        fs::remove_dir_all(home).unwrap();
    }
}

fn stable_digest(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}
