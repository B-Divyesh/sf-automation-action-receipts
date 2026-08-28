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
