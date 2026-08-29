use std::fs;
use std::process::Command;

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_action-receipts"))
}

#[test]
fn cli_creates_seals_and_verifies_json_and_html() {
    let dir = tempfile::tempdir().unwrap();
    let receipt = dir.path().join("deploy.receipt.json");
    let html = dir.path().join("deploy.receipt.html");

    let created = cli()
        .args([
            "new",
            "--out",
            receipt.to_str().unwrap(),
            "--actor",
            "release-bot@ci",
            "--authorization",
            "change-482 approved by ops",
            "--summary",
            "Publish docs",
            "--scope",
            "repo:docs/**",
            "--retention-days",
            "30",
        ])
        .output()
        .unwrap();
    assert!(
        created.status.success(),
        "{}",
        String::from_utf8_lossy(&created.stderr)
    );

    let recorded = cli()
        .args([
            "record",
            "--receipt",
            receipt.to_str().unwrap(),
            "--kind",
            "tool",
            "--tool",
            "git",
            "--input-json",
            r#"{"operation":"push","token":"must-not-survive","ratio":1.25}"#,
            "--output-json",
            r#"{"commit":"abc123"}"#,
        ])
        .output()
        .unwrap();
    assert!(
        recorded.status.success(),
        "{}",
        String::from_utf8_lossy(&recorded.stderr)
    );
    assert!(!fs::read_to_string(&receipt)
        .unwrap()
        .contains("must-not-survive"));

    let artifact = dir.path().join("artifact.txt");
    fs::write(&artifact, "build output").unwrap();
    let ran = cli()
        .args([
            "run",
            "--receipt",
            receipt.to_str().unwrap(),
            "--artifact",
            artifact.to_str().unwrap(),
            "--redact",
            "private-value",
            "--",
            "sh",
            "-c",
            "printf private-value",
        ])
        .output()
        .unwrap();
    assert!(
        ran.status.success(),
        "{}",
        String::from_utf8_lossy(&ran.stderr)
    );
    assert!(!fs::read_to_string(&receipt)
        .unwrap()
        .contains("private-value"));

    let sealed = cli()
        .args([
            "seal",
            "--receipt",
            receipt.to_str().unwrap(),
            "--html",
            html.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        sealed.status.success(),
        "{}",
        String::from_utf8_lossy(&sealed.stderr)
    );

    for path in [&receipt, &html] {
        let verified = cli()
            .args(["verify", path.to_str().unwrap(), "--json"])
            .output()
            .unwrap();
        assert!(
            verified.status.success(),
            "{}",
            String::from_utf8_lossy(&verified.stderr)
        );
        let result: serde_json::Value = serde_json::from_slice(&verified.stdout).unwrap();
        assert_eq!(result["valid"], true);
        assert_eq!(result["event_count"], 2);
    }
}

// @claim:cli-demo-lifecycle
#[test]
fn claim_cli_demo_lifecycle_creates_isolated_signed_outputs() {
    let output = cli().arg("demo").output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json = stdout
        .lines()
        .find_map(|line| line.strip_prefix("JSON: "))
        .unwrap();
    let html = stdout
        .lines()
        .find_map(|line| line.strip_prefix("HTML: "))
        .unwrap();
    for path in [json, html] {
        let verified = cli().args(["verify", path, "--json"]).output().unwrap();
        assert!(
            verified.status.success(),
            "{}",
            String::from_utf8_lossy(&verified.stderr)
        );
    }
    let _ = fs::remove_dir_all(std::path::Path::new(json).parent().unwrap());
}

// @claim:cli-no-account
#[test]
fn claim_cli_demo_needs_no_account_or_environment_credentials() {
    let output = Command::new(env!("CARGO_BIN_EXE_action-receipts"))
        .env_clear()
        .arg("demo")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json = stdout
        .lines()
        .find_map(|line| line.strip_prefix("JSON: "))
        .unwrap();
    let _ = fs::remove_dir_all(std::path::Path::new(json).parent().unwrap());
}

// @claim:declared-boundary-fields
#[test]
fn claim_declared_boundary_fields_are_written_before_events() {
    let output = cli().arg("demo").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json = stdout
        .lines()
        .find_map(|line| line.strip_prefix("JSON: "))
        .unwrap();
    let receipt: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(json).unwrap()).unwrap();
    assert_eq!(receipt["subject"]["actor"], "release-bot@ci");
    assert_eq!(
        receipt["subject"]["authorization"],
        "change-482 approved by operations"
    );
    assert_eq!(receipt["subject"]["scope"][0], "repo:docs/**");
    assert_eq!(receipt["policy"]["retention_days"], 30);
    assert_eq!(receipt["events"].as_array().unwrap().len(), 2);
    let _ = fs::remove_dir_all(std::path::Path::new(json).parent().unwrap());
}

// @claim:command-provenance
#[test]
fn claim_command_provenance_records_command_output_and_artifact_hash() {
    let dir = tempfile::tempdir().unwrap();
    let receipt = dir.path().join("receipt.json");
    let artifact = dir.path().join("output.txt");
    fs::write(&artifact, "artifact contents").unwrap();
    assert!(cli()
        .args([
            "new",
            "--out",
            receipt.to_str().unwrap(),
            "--actor",
            "bot",
            "--authorization",
            "approved",
            "--summary",
            "test",
            "--scope",
            "repo:**"
        ])
        .status()
        .unwrap()
        .success());
    assert!(cli()
        .args([
            "run",
            "--receipt",
            receipt.to_str().unwrap(),
            "--artifact",
            artifact.to_str().unwrap(),
            "--",
            "sh",
            "-c",
            "printf done"
        ])
        .status()
        .unwrap()
        .success());
    let value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(receipt).unwrap()).unwrap();
    let event = &value["events"][0];
    assert_eq!(event["command"][0], "sh");
    assert_eq!(event["output"]["stdout"]["text"], "done");
    assert_eq!(event["exit_code"], 0);
    assert!(event["output"]["duration_ms"].as_u64().is_some());
    assert_eq!(event["artifacts"][0]["sha256"].as_str().unwrap().len(), 64);
}

// @claim:redact-before-storage
#[test]
fn claim_redact_before_storage_removes_literal_and_default_key_secrets() {
    let dir = tempfile::tempdir().unwrap();
    let receipt = dir.path().join("receipt.json");
    assert!(cli()
        .args([
            "new",
            "--out",
            receipt.to_str().unwrap(),
            "--actor",
            "bot",
            "--authorization",
            "approved",
            "--summary",
            "test",
            "--scope",
            "repo:**"
        ])
        .status()
        .unwrap()
        .success());
    assert!(cli()
        .args([
            "record",
            "--receipt",
            receipt.to_str().unwrap(),
            "--tool",
            "fixture",
            "--input-json",
            r#"{"token":"default-secret"}"#,
            "--output-json",
            r#"{"value":"literal-secret"}"#,
            "--redact",
            "literal-secret"
        ])
        .status()
        .unwrap()
        .success());
    let saved = fs::read_to_string(receipt).unwrap();
    assert!(!saved.contains("default-secret"));
    assert!(!saved.contains("literal-secret"));
    assert!(saved.contains("[REDACTED]"));
}

// @claim:json-html-export
#[test]
fn claim_json_html_export_embeds_the_signed_receipt() {
    let output = cli().arg("demo").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json = stdout
        .lines()
        .find_map(|line| line.strip_prefix("JSON: "))
        .unwrap();
    let html = stdout
        .lines()
        .find_map(|line| line.strip_prefix("HTML: "))
        .unwrap();
    let json_receipt = action_receipts::read_receipt(std::path::Path::new(json)).unwrap();
    let html_receipt = action_receipts::read_receipt(std::path::Path::new(html)).unwrap();
    assert_eq!(json_receipt, html_receipt);
    assert!(fs::read_to_string(html)
        .unwrap()
        .contains("type=\"application/json\""));
    let _ = fs::remove_dir_all(std::path::Path::new(json).parent().unwrap());
}

// @claim:private-key-permissions
#[test]
fn claim_private_key_is_separate_and_private() {
    let dir = tempfile::tempdir().unwrap();
    let receipt = dir.path().join("receipt.json");
    assert!(cli()
        .args([
            "new",
            "--out",
            receipt.to_str().unwrap(),
            "--actor",
            "bot",
            "--authorization",
            "approved",
            "--summary",
            "test",
            "--scope",
            "repo:**"
        ])
        .status()
        .unwrap()
        .success());
    let key = std::path::PathBuf::from(format!("{}.key", receipt.display()));
    assert!(key.exists());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(key).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
}

// @claim:unknown-fields-rejected
#[test]
fn claim_unknown_fields_are_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bad.json");
    fs::write(
        &path,
        r#"{"format":"https://actionreceipts.dev/receipt/v1","unexpected":true}"#,
    )
    .unwrap();
    let result = cli()
        .args(["verify", path.to_str().unwrap(), "--json"])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("unknown field"));
}
