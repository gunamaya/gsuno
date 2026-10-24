//! The semantic exit-code contract (0-4): deterministic triggers via the
//! hidden `contract` command, plus the natural parse and command paths.
//! Code 5 no longer exists (breaking vs 0.5.x — not-found is now 3).

mod common;
use common::{gsuno, gsuno_in};

// ── Contract command: deterministic 0-4 ────────────────────────────────────

#[test]
fn contract_exit_0() {
    gsuno().args(["contract", "0"]).assert().code(0);
}

#[test]
fn contract_exit_1_transient() {
    gsuno().args(["contract", "1"]).assert().code(1);
}

#[test]
fn contract_exit_2_config() {
    gsuno().args(["contract", "2"]).assert().code(2);
}

#[test]
fn contract_exit_3_bad_input() {
    gsuno().args(["contract", "3"]).assert().code(3);
}

#[test]
fn contract_exit_4_rate_limited() {
    gsuno().args(["contract", "4"]).assert().code(4);
}

#[test]
fn contract_unknown_code_is_bad_input() {
    // The removed code 5 must not resurface via the contract hook.
    gsuno().args(["contract", "5"]).assert().code(3);
}

// ── Natural paths ──────────────────────────────────────────────────────────

#[test]
fn help_exits_0() {
    gsuno().arg("--help").assert().code(0);
}

#[test]
fn version_exits_0() {
    gsuno().arg("--version").assert().code(0);
}

#[test]
fn missing_subcommand_exits_3() {
    gsuno().assert().code(3);
}

#[test]
fn unknown_flag_exits_3() {
    gsuno()
        .args(["list", "--definitely-not-a-flag"])
        .assert()
        .code(3);
}

// ── Required positionals: an empty ID list is bad input, not an auth call ────

#[test]
fn status_without_ids_exits_3() {
    // Missing required IDs must fail at parse (exit 3), never reach the API.
    gsuno().arg("status").assert().code(3);
}

#[test]
fn download_without_ids_exits_3() {
    gsuno().arg("download").assert().code(3);
}

#[test]
fn publish_without_ids_exits_3() {
    gsuno().arg("publish").assert().code(3);
}

// ── Dead flags stay dead ─────────────────────────────────────────────────────

#[test]
fn stems_wait_flag_removed_exits_3() {
    // stems accepted but ignored --wait; the flag is gone, so passing it is an
    // unknown-flag parse error rather than a silent no-op.
    gsuno().args(["stems", "abc", "--wait"]).assert().code(3);
}

#[test]
fn concat_wait_flag_removed_exits_3() {
    gsuno().args(["concat", "abc", "--wait"]).assert().code(3);
}

#[test]
fn agent_info_exits_0() {
    gsuno().arg("agent-info").assert().code(0);
}

#[test]
fn config_show_and_path_exit_0() {
    let tmp = tempfile::tempdir().unwrap();
    gsuno_in(tmp.path())
        .args(["config", "show"])
        .assert()
        .code(0);
    gsuno_in(tmp.path())
        .args(["config", "path"])
        .assert()
        .code(0);
}

#[test]
fn doctor_without_auth_exits_2() {
    // Fresh home: auth_file is a failing check, so doctor reports config
    // error (2) — the fix is `gsuno auth --login`, not a retry.
    let tmp = tempfile::tempdir().unwrap();
    let out = gsuno_in(tmp.path()).arg("doctor").output().unwrap();
    assert_eq!(out.status.code(), Some(2));

    // Failed commands write no success-shaped data to stdout. The complete
    // diagnostic report is attached to the one stderr error envelope.
    assert!(out.stdout.is_empty());
    let stderr = std::str::from_utf8(&out.stderr).unwrap();
    assert_eq!(stderr.lines().count(), 1, "{stderr}");
    let envelope: serde_json::Value = serde_json::from_str(stderr).unwrap();
    assert_eq!(envelope["status"], "error");
    assert_eq!(envelope["error"]["code"], "config_error");
    let report = &envelope["error"]["details"];
    assert!(report["checks"].is_array());
    let auth_check = report["checks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["name"] == "auth_file")
        .expect("doctor must include an auth_file check");
    assert_eq!(auth_check["status"], "fail");
}
