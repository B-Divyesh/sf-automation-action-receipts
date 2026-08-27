use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, SecondsFormat, Utc};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub const FORMAT: &str = "https://actionreceipts.dev/receipt/v1";
pub const REDACTED: &str = "[REDACTED]";
pub const MAX_CAPTURE_BYTES: usize = 65_536;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub format: String,
    pub receipt_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub state: ReceiptState,
    pub subject: Subject,
    pub policy: Policy,
    pub events: Vec<Event>,
    pub chain_head: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Proof>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReceiptState {
    Open,
    Sealed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Subject {
    pub summary: String,
    pub actor: String,
    pub authorization: String,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub retention_days: u32,
    pub default_sensitive_keys: bool,
    pub redact_environment: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub sequence: u64,
    pub timestamp: String,
    pub kind: String,
    pub tool: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    pub input: Value,
    pub output: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    pub artifacts: Vec<Artifact>,
    pub previous_hash: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Proof {
    pub algorithm: String,
    pub public_key: String,
    pub signed_at: String,
    pub bundle_sha256: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyFile {
    pub algorithm: String,
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Verification {
    pub valid: bool,
    pub receipt_id: Option<String>,
    pub event_count: usize,
    pub chain_valid: bool,
    pub signature_valid: bool,
    pub bundle_sha256: Option<String>,
    pub message: String,
}

#[derive(Serialize)]
struct SigningPayload<'a> {
    format: &'a str,
    receipt_id: &'a str,
    created_at: &'a str,
    updated_at: &'a str,
    state: &'a ReceiptState,
    subject: &'a Subject,
    policy: &'a Policy,
    events: &'a [Event],
    chain_head: &'a str,
}

#[derive(Serialize)]
struct EventPayload<'a> {
    sequence: u64,
    timestamp: &'a str,
    kind: &'a str,
    tool: &'a str,
    command: &'a Option<Vec<String>>,
    input: &'a Value,
    output: &'a Value,
    exit_code: &'a Option<i32>,
    artifacts: &'a [Artifact],
    previous_hash: &'a str,
}

pub fn now() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}

pub fn new_receipt(subject: Subject, policy: Policy) -> Receipt {
    let timestamp = now();
    let mut random = [0u8; 16];
    OsRng.fill_bytes(&mut random);
    Receipt {
        format: FORMAT.to_string(),
        receipt_id: format!("ar_{}", hex::encode(random)),
        created_at: timestamp.clone(),
        updated_at: timestamp,
        state: ReceiptState::Open,
        subject,
        policy,
        events: Vec::new(),
        chain_head: "0".repeat(64),
        proof: None,
    }
}

pub fn generate_key() -> KeyFile {
    let signing = SigningKey::generate(&mut OsRng);
    KeyFile {
        algorithm: "Ed25519".into(),
        private_key: BASE64.encode(signing.to_bytes()),
        public_key: BASE64.encode(signing.verifying_key().to_bytes()),
    }
}

pub fn read_receipt(path: &Path) -> Result<Receipt, String> {
    let text =
        fs::read_to_string(path).map_err(|e| format!("could not read {}: {e}", path.display()))?;
    parse_receipt_text(&text)
}

pub fn parse_receipt_text(text: &str) -> Result<Receipt, String> {
    let json = if text.trim_start().starts_with('<') {
        extract_embedded_receipt(text)?
    } else {
        text.to_string()
    };
    serde_json::from_str(&json).map_err(|e| format!("invalid receipt JSON: {e}"))
}

pub fn write_receipt(path: &Path, receipt: &Receipt) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(receipt).map_err(|e| e.to_string())?;
    atomic_write(path, &bytes).map_err(|e| format!("could not write {}: {e}", path.display()))
}

pub fn write_key(path: &Path, key: &KeyFile) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(key).map_err(|e| e.to_string())?;
    atomic_write(path, &bytes)
        .map_err(|e| format!("could not write key {}: {e}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .map_err(|e| format!("could not protect key {}: {e}", path.display()))?;
    }
    Ok(())
}

fn atomic_write(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    let temp = path.with_extension(format!(
        "{}.tmp",
        path.extension().and_then(|x| x.to_str()).unwrap_or("file")
    ));
    fs::write(&temp, bytes)?;
    fs::rename(temp, path)
}

pub fn default_key_path(receipt: &Path) -> PathBuf {
    PathBuf::from(format!("{}.key", receipt.display()))
}

pub fn redact_value(value: &mut Value, literals: &[String], env_names: &[String]) {
    let mut secrets: Vec<String> = literals.iter().filter(|v| !v.is_empty()).cloned().collect();
    for name in env_names {
        if let Ok(value) = std::env::var(name) {
            if !value.is_empty() {
                secrets.push(value);
            }
        }
    }
    redact_inner(value, &secrets);
}

fn redact_inner(value: &mut Value, secrets: &[String]) {
    match value {
        Value::Object(map) => {
            for (key, value) in map.iter_mut() {
                if is_sensitive_key(key) {
                    *value = Value::String(REDACTED.into());
                } else {
                    redact_inner(value, secrets);
                }
            }
        }
        Value::Array(values) => values.iter_mut().for_each(|v| redact_inner(v, secrets)),
        Value::String(text) => {
            for secret in secrets {
                *text = text.replace(secret, REDACTED);
            }
        }
        _ => {}
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase().replace('-', "_");
    [
        "password",
        "passwd",
        "secret",
        "token",
        "api_key",
        "apikey",
        "authorization",
        "private_key",
        "cookie",
    ]
    .iter()
    .any(|needle| key == *needle || key.ends_with(&format!("_{needle}")))
}

pub fn artifact_for(path: &Path) -> Result<Artifact, String> {
    let mut file = fs::File::open(path)
        .map_err(|e| format!("could not read artifact {}: {e}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut bytes = 0u64;
    let mut buffer = [0u8; 32 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|e| format!("could not hash artifact {}: {e}", path.display()))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
        bytes += count as u64;
    }
    Ok(Artifact {
        path: path.to_string_lossy().into_owned(),
        sha256: hex::encode(hasher.finalize()),
        bytes,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn append_event(
    receipt: &mut Receipt,
    kind: String,
    tool: String,
    command: Option<Vec<String>>,
    mut input: Value,
    mut output: Value,
    exit_code: Option<i32>,
    artifacts: Vec<Artifact>,
    literals: &[String],
) -> Result<(), String> {
    if receipt.state != ReceiptState::Open {
        return Err("receipt is sealed; start a new receipt for additional events".into());
    }
    verify_chain(receipt)?;
    redact_value(&mut input, literals, &receipt.policy.redact_environment);
    redact_value(&mut output, literals, &receipt.policy.redact_environment);
    let command = command.map(|parts| {
        parts
            .into_iter()
            .map(|mut part| {
                for literal in literals {
                    if !literal.is_empty() {
                        part = part.replace(literal, REDACTED);
                    }
                }
                for env_name in &receipt.policy.redact_environment {
                    if let Ok(secret) = std::env::var(env_name) {
                        if !secret.is_empty() {
                            part = part.replace(&secret, REDACTED);
                        }
                    }
                }
                part
            })
            .collect()
    });
    let mut event = Event {
        sequence: receipt.events.len() as u64 + 1,
        timestamp: now(),
        kind,
        tool,
        command,
        input,
        output,
        exit_code,
        artifacts,
        previous_hash: receipt.chain_head.clone(),
        hash: String::new(),
    };
    event.hash = event_hash(&event)?;
    receipt.chain_head = event.hash.clone();
    receipt.events.push(event);
    receipt.updated_at = now();
    Ok(())
}

pub fn event_hash(event: &Event) -> Result<String, String> {
    let payload = EventPayload {
        sequence: event.sequence,
        timestamp: &event.timestamp,
        kind: &event.kind,
        tool: &event.tool,
        command: &event.command,
        input: &event.input,
        output: &event.output,
        exit_code: &event.exit_code,
        artifacts: &event.artifacts,
        previous_hash: &event.previous_hash,
    };
    let bytes = serde_jcs::to_vec(&payload).map_err(|e| e.to_string())?;
    Ok(sha256(&bytes))
}

pub fn verify_chain(receipt: &Receipt) -> Result<(), String> {
    let mut previous = "0".repeat(64);
    for (index, event) in receipt.events.iter().enumerate() {
        if event.sequence != index as u64 + 1 {
            return Err(format!("event {} has an invalid sequence", index + 1));
        }
        if event.previous_hash != previous {
            return Err(format!(
                "event {} does not link to the previous event",
                index + 1
            ));
        }
        let expected = event_hash(event)?;
        if event.hash != expected {
            return Err(format!(
                "event {} hash does not match its contents",
                index + 1
            ));
        }
        previous = event.hash.clone();
    }
    if receipt.chain_head != previous {
        return Err("chain head does not match the final event".into());
    }
    Ok(())
}

fn signing_bytes(receipt: &Receipt) -> Result<Vec<u8>, String> {
    serde_jcs::to_vec(&SigningPayload {
        format: &receipt.format,
        receipt_id: &receipt.receipt_id,
        created_at: &receipt.created_at,
        updated_at: &receipt.updated_at,
        state: &receipt.state,
        subject: &receipt.subject,
        policy: &receipt.policy,
        events: &receipt.events,
        chain_head: &receipt.chain_head,
    })
    .map_err(|e| e.to_string())
}

pub fn seal(receipt: &mut Receipt, key: &KeyFile) -> Result<(), String> {
    if receipt.state != ReceiptState::Open {
        return Err("receipt is already sealed".into());
    }
    if receipt.events.is_empty() {
        return Err("cannot seal an empty receipt; record at least one event".into());
    }
    verify_chain(receipt)?;
    if key.algorithm != "Ed25519" {
        return Err("unsupported key algorithm".into());
    }
    let private_bytes = BASE64
        .decode(&key.private_key)
        .map_err(|_| "key contains invalid base64")?;
    let private: [u8; 32] = private_bytes
        .try_into()
        .map_err(|_| "Ed25519 private key must be 32 bytes")?;
    let signing = SigningKey::from_bytes(&private);
    if BASE64.encode(signing.verifying_key().to_bytes()) != key.public_key {
        return Err("key file public/private key mismatch".into());
    }
    receipt.state = ReceiptState::Sealed;
    receipt.updated_at = now();
    let bytes = signing_bytes(receipt)?;
    let signature = signing.sign(&bytes);
    receipt.proof = Some(Proof {
        algorithm: "Ed25519".into(),
        public_key: key.public_key.clone(),
        signed_at: receipt.updated_at.clone(),
        bundle_sha256: sha256(&bytes),
        signature: BASE64.encode(signature.to_bytes()),
    });
    Ok(())
}

pub fn verify(receipt: &Receipt) -> Verification {
    let id = Some(receipt.receipt_id.clone());
    if receipt.format != FORMAT {
        return invalid(
            id,
            receipt.events.len(),
            false,
            false,
            "unsupported receipt format",
        );
    }
    if let Err(message) = verify_chain(receipt) {
        return invalid(id, receipt.events.len(), false, false, &message);
    }
    if receipt.state != ReceiptState::Sealed {
        return invalid(
            id,
            receipt.events.len(),
            true,
            false,
            "receipt is open and has no signature",
        );
    }
    let Some(proof) = &receipt.proof else {
        return invalid(
            id,
            receipt.events.len(),
            true,
            false,
            "sealed receipt has no proof",
        );
    };
    if proof.algorithm != "Ed25519" {
        return invalid(
            id,
            receipt.events.len(),
            true,
            false,
            "unsupported signature algorithm",
        );
    }
    let bytes = match signing_bytes(receipt) {
        Ok(v) => v,
        Err(e) => return invalid(id, receipt.events.len(), true, false, &e),
    };
    let bundle = sha256(&bytes);
    if bundle != proof.bundle_sha256 {
        return invalid(
            id,
            receipt.events.len(),
            true,
            false,
            "bundle digest does not match signed contents",
        );
    }
    let public = match BASE64
        .decode(&proof.public_key)
        .ok()
        .and_then(|v| <[u8; 32]>::try_from(v).ok())
        .and_then(|v| VerifyingKey::from_bytes(&v).ok())
    {
        Some(v) => v,
        None => {
            return invalid(
                id,
                receipt.events.len(),
                true,
                false,
                "invalid Ed25519 public key",
            )
        }
    };
    let signature = match BASE64
        .decode(&proof.signature)
        .ok()
        .and_then(|v| <[u8; 64]>::try_from(v).ok())
        .map(|v| Signature::from_bytes(&v))
    {
        Some(v) => v,
        None => {
            return invalid(
                id,
                receipt.events.len(),
                true,
                false,
                "invalid Ed25519 signature encoding",
            )
        }
    };
    if public.verify(&bytes, &signature).is_err() {
        return invalid(
            id,
            receipt.events.len(),
            true,
            false,
            "signature does not match signed contents",
        );
    }
    Verification {
        valid: true,
        receipt_id: id,
        event_count: receipt.events.len(),
        chain_valid: true,
        signature_valid: true,
        bundle_sha256: Some(bundle),
        message: "receipt chain and Ed25519 signature are valid".into(),
    }
}

fn invalid(
    id: Option<String>,
    count: usize,
    chain: bool,
    signature: bool,
    message: &str,
) -> Verification {
    Verification {
        valid: false,
        receipt_id: id,
        event_count: count,
        chain_valid: chain,
        signature_valid: signature,
        bundle_sha256: None,
        message: message.into(),
    }
}

fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

pub fn html_report(receipt: &Receipt) -> Result<String, String> {
    let json = serde_json::to_string(receipt)
        .map_err(|e| e.to_string())?
        .replace("</script", "<\\/script");
    let verification = verify(receipt);
    let status = if verification.valid {
        "VALID"
    } else {
        "INVALID"
    };
    let mut event_rows = String::new();
    for event in &receipt.events {
        event_rows.push_str(&format!(
            "<li><span class=seq>{:02}</span><div><strong>{}</strong><p>{} · {}</p><code>{}</code></div></li>",
            event.sequence, html_escape(&event.kind), html_escape(&event.tool), html_escape(&event.timestamp), html_escape(&event.hash)
        ));
    }
    Ok(format!(
        r#"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Action Receipt {id}</title><style>:root{{--paper:#f4f0e6;--sheet:#fffdf7;--ink:#171713;--proof:#b9f227;--signal:#ff5c35}}*{{box-sizing:border-box}}body{{margin:0;background:var(--paper);color:var(--ink);font:16px/1.55 ui-monospace,monospace}}main{{width:min(900px,calc(100% - 32px));margin:40px auto 80px}}header{{border:3px solid;padding:24px;background:var(--sheet);box-shadow:7px 7px 0 var(--ink)}}h1{{font:clamp(34px,7vw,68px)/.95 Arial Black,sans-serif;margin:8px 0 18px}}.status{{display:inline-block;background:var(--proof);border:2px solid;padding:8px 12px;font-weight:800}}dl{{display:grid;grid-template-columns:180px 1fr;gap:8px;margin:28px 0}}dt{{font-weight:800}}dd{{margin:0;overflow-wrap:anywhere}}section{{margin-top:48px}}li{{display:grid;grid-template-columns:56px 1fr;gap:16px;border-top:2px solid;padding:18px 0}}.seq{{font:28px Arial Black,sans-serif;color:#3b5bdb}}p{{margin:4px 0}}code{{font-size:12px;overflow-wrap:anywhere}}.warning{{border-left:10px solid var(--signal);padding:12px 16px;background:var(--sheet)}}@media(max-width:560px){{dl{{grid-template-columns:1fr}}main{{margin-top:16px}}}}</style></head><body><main><header><span>ACTION RECEIPT / V1</span><h1>{summary}</h1><span class=status>✓ {status}</span></header><dl><dt>Receipt ID</dt><dd>{id}</dd><dt>Actor (declared)</dt><dd>{actor}</dd><dt>Authorization</dt><dd>{authorization}</dd><dt>Scope</dt><dd>{scope}</dd><dt>Signed</dt><dd>{signed}</dd><dt>Chain head</dt><dd><code>{head}</code></dd></dl><p class=warning><strong>Integrity, not intent.</strong> A valid signature proves this bundle is unchanged. It does not prove identity, authorization legitimacy, or correctness.</p><section><h2>Recorded events ({count})</h2><ol>{events}</ol></section><script id="action-receipt" type="application/json">{json}</script></main></body></html>"#,
        id = html_escape(&receipt.receipt_id),
        summary = html_escape(&receipt.subject.summary),
        actor = html_escape(&receipt.subject.actor),
        authorization = html_escape(&receipt.subject.authorization),
        scope = html_escape(&receipt.subject.scope.join(", ")),
        signed = html_escape(
            receipt
                .proof
                .as_ref()
                .map(|p| p.signed_at.as_str())
                .unwrap_or("not sealed")
        ),
        head = html_escape(&receipt.chain_head),
        count = receipt.events.len(),
        events = event_rows,
        json = json,
        status = status
    ))
}

pub fn extract_embedded_receipt(html: &str) -> Result<String, String> {
    let marker = "<script id=\"action-receipt\" type=\"application/json\">";
    let start = html
        .find(marker)
        .ok_or("HTML does not contain an embedded Action Receipt")?
        + marker.len();
    let end = html[start..]
        .find("</script>")
        .ok_or("embedded receipt script is not closed")?
        + start;
    Ok(html[start..end].replace("<\\/script", "</script"))
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn older_than(receipt: &Receipt, days: i64) -> Result<bool, String> {
    let created = DateTime::parse_from_rfc3339(&receipt.created_at)
        .map_err(|e| format!("invalid created_at: {e}"))?;
    Ok(created < Utc::now() - chrono::Duration::days(days))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn subject() -> Subject {
        Subject {
            summary: "Deploy docs".into(),
            actor: "ci-bot".into(),
            authorization: "ticket-42".into(),
            scope: vec!["repo:docs/**".into()],
        }
    }

    #[test]
    fn documented_lifecycle_verifies_and_detects_tampering() {
        let mut receipt = new_receipt(
            subject(),
            Policy {
                retention_days: 30,
                default_sensitive_keys: true,
                redact_environment: vec![],
            },
        );
        append_event(
            &mut receipt,
            "tool".into(),
            "git".into(),
            None,
            serde_json::json!({"operation":"push"}),
            serde_json::json!({"commit":"abc123"}),
            Some(0),
            vec![],
            &[],
        )
        .unwrap();
        seal(&mut receipt, &generate_key()).unwrap();
        assert!(verify(&receipt).valid);
        receipt.events[0].output["commit"] = Value::String("changed".into());
        let result = verify(&receipt);
        assert!(!result.valid);
        assert!(!result.chain_valid);
    }

    #[test]
    fn redacts_sensitive_keys_and_literals_before_hashing() {
        let mut value = serde_json::json!({"api_key":"never-store", "message":"token=needle", "nested":{"password":"bad"}});
        redact_value(&mut value, &["needle".into()], &[]);
        assert_eq!(value["api_key"], REDACTED);
        assert_eq!(value["nested"]["password"], REDACTED);
        assert_eq!(value["message"], "token=[REDACTED]");
        assert!(!value.to_string().contains("never-store"));
    }

    #[test]
    fn html_round_trip_keeps_signed_receipt() {
        let mut receipt = new_receipt(
            subject(),
            Policy {
                retention_days: 7,
                default_sensitive_keys: true,
                redact_environment: vec![],
            },
        );
        append_event(
            &mut receipt,
            "command".into(),
            "printf".into(),
            Some(vec!["printf".into(), "ok".into()]),
            Value::Null,
            serde_json::json!({"stdout":"ok"}),
            Some(0),
            vec![],
            &[],
        )
        .unwrap();
        seal(&mut receipt, &generate_key()).unwrap();
        let parsed = parse_receipt_text(&html_report(&receipt).unwrap()).unwrap();
        assert!(verify(&parsed).valid);
        assert_eq!(parsed.receipt_id, receipt.receipt_id);
    }

    #[test]
    fn hashes_artifacts() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("artifact.txt");
        fs::write(&path, b"receipt").unwrap();
        let artifact = artifact_for(&path).unwrap();
        assert_eq!(artifact.bytes, 7);
        assert_eq!(
            artifact.sha256,
            "6f32860910ca0fb2a20c7fda143666b09dbf8db5238195c90a586fb542ff0cad"
        );
    }

    #[test]
    fn empty_receipts_cannot_be_sealed() {
        let mut receipt = new_receipt(
            subject(),
            Policy {
                retention_days: 1,
                default_sensitive_keys: true,
                redact_environment: vec![],
            },
        );
        assert!(seal(&mut receipt, &generate_key())
            .unwrap_err()
            .contains("empty"));
    }
}
