#[cfg(unix)]
#[test]
fn routing_requires_explicit_action_and_failures_stop_playback() {
    use std::{fs, os::unix::fs::PermissionsExt, process::Command};
    let root = std::env::temp_dir().join(format!(
        "music-routing-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&root).unwrap();
    for (name, body) in [
        ("system_profiler", "exit 1"),
        (
            "swift",
            "printf '%s\\n' \"$1\" >> \"$TEST_ROUTING_LOG\"; exit \"${TEST_SWIFT_EXIT:-0}\"",
        ),
        (
            "afplay",
            "printf 'played\\n' >> \"$TEST_PLAY_LOG\"; exit \"${TEST_PLAY_EXIT:-0}\"",
        ),
        ("open", "exit 0"),
    ] {
        let path = root.join(name);
        fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
    let bridge = root.join("fixture.swift");
    fs::write(&bridge, "// fixture; only the stub runner sees this file\n").unwrap();
    let route_log = root.join("routes.log");
    let play_log = root.join("play.log");
    let run = |mode: &str, flags: &[&str], swift_exit: &str, play_exit: &str| {
        Command::new(env!("CARGO_BIN_EXE_8b-is-music"))
            .args([mode, "--layers", "64", "--seconds", "0.001", "--out"])
            .arg(root.join("test.wav"))
            .arg("--routing-script")
            .arg(&bridge)
            .args(flags)
            .env("PATH", &root)
            .env("TEST_ROUTING_LOG", &route_log)
            .env("TEST_PLAY_LOG", &play_log)
            .env("TEST_SWIFT_EXIT", swift_exit)
            .env("TEST_PLAY_EXIT", play_exit)
            .output()
            .unwrap()
    };
    for mode in ["render", "dj", "youtube", "bluetooth"] {
        let output = run(mode, &["--play"], "0", "0");
        assert!(
            output.status.success(),
            "{mode}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !route_log.exists(),
            "{mode} configured routing without --apply-live"
        );
    }
    for mode in ["render", "dj", "youtube", "bluetooth"] {
        let output = run(mode, &["--apply-live", "--json"], "0", "0");
        assert!(
            output.status.success(),
            "{mode}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        if mode == "bluetooth" || mode == "dj" {
            serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap();
        }
    }
    assert_eq!(fs::read_to_string(&route_log).unwrap().lines().count(), 4);
    fs::remove_file(&play_log).unwrap();
    for mode in ["render", "dj", "youtube", "bluetooth"] {
        let output = run(mode, &["--apply-live", "--play", "--json"], "7", "0");
        assert_eq!(output.status.code(), Some(1));
        assert!(
            !play_log.exists(),
            "playback occurred after routing failure"
        );
    }
    let output = run("render", &["--play"], "0", "9");
    assert_eq!(output.status.code(), Some(1));
    fs::remove_file(&bridge).unwrap();
    let output = run("bluetooth", &["--apply-live", "--json"], "0", "0");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    fs::remove_dir_all(root).unwrap();
}
