//! Native peer security is independent of the currently Linux-only process identity schema.
#![cfg(any(target_os = "linux", target_os = "macos"))]
use std::io::{Read, Write};
use std::os::fd::FromRawFd;
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::{Command, Stdio};
use yai_sdk::platform::{peer_credentials, verify_peer_same_user};

#[test]
fn same_process_socket_pair_has_native_uid_and_pid() {
    let (left, right) = UnixStream::pair().unwrap();
    for stream in [&left, &right] {
        let peer = peer_credentials(stream).unwrap();
        assert_eq!(peer.uid, unsafe { libc::geteuid() });
        assert_eq!(peer.pid, std::process::id());
        verify_peer_same_user(stream).unwrap();
    }
}

#[test]
fn tcp_socket_cannot_supply_unix_peer_credentials() {
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd >= 0);
    // The owned descriptor is deliberately not a Unix socket; native identity must refuse it.
    let stream = unsafe { UnixStream::from_raw_fd(fd) };
    assert!(peer_credentials(&stream).is_err());
    assert!(verify_peer_same_user(&stream).is_err());
}

#[test]
fn peer_child_helper() {
    let Some(path) = std::env::var_os("YAI_SDK_NATIVE_PEER_SOCKET") else {
        return;
    };
    let mut stream = UnixStream::connect(path).unwrap();
    stream.write_all(&[1]).unwrap();
    let mut done = [0];
    stream.read_exact(&mut done).unwrap();
}

#[test]
fn accepted_socket_identifies_the_connecting_child_not_the_listener() {
    let root = std::env::temp_dir().join(format!("yai-sdk-native-peer-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let path = root.join("peer.sock");
    let listener = UnixListener::bind(&path).unwrap();
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "peer_child_helper", "--nocapture"])
        .env("YAI_SDK_NATIVE_PEER_SOCKET", &path)
        .stdout(Stdio::null())
        .spawn()
        .unwrap();
    let (mut stream, _) = listener.accept().unwrap();
    let mut ready = [0];
    stream.read_exact(&mut ready).unwrap();
    let peer = peer_credentials(&stream).unwrap();
    assert_eq!(peer.pid, child.id());
    assert_ne!(peer.pid, std::process::id());
    assert_eq!(peer.uid, unsafe { libc::geteuid() });
    verify_peer_same_user(&stream).unwrap();
    stream.write_all(&[1]).unwrap();
    assert!(child.wait().unwrap().success());
    drop(stream);
    drop(listener);
    std::fs::remove_file(path).unwrap();
    std::fs::remove_dir(root).unwrap();
}

#[test]
#[cfg(target_os = "macos")]
fn process_identity_still_refuses_without_a_native_schema() {
    assert_eq!(
        yai_sdk::LocalProcessIdentity::capture(std::process::id()).unwrap_err(),
        "process_identity_unsupported_platform"
    );
}
