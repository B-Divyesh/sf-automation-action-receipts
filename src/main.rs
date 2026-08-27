use action_receipts::*;
use clap::{Args, Parser, Subcommand};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::time::Instant;

#[derive(Parser)]
#[command(
    name = "action-receipts",
    version,
    about = "Create portable, signed evidence for automated actions",
    long_about = "Create hash-chained JSON/HTML receipts that record declared authorization, scope, commands, inputs, outputs, and artifact hashes. Verification is offline. Signatures prove bundle integrity, not identity, intent, or correctness."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an open receipt and generate its separate Ed25519 key
    New(NewArgs),
    /// Append a structured event declared by an integration
    Record(RecordArgs),
    /// Execute a command and append its captured result
    Run(RunArgs),
    /// Lock and sign a non-empty receipt; optionally emit readable HTML
    Seal(SealArgs),
    /// Verify a signed JSON or HTML receipt offline
    Verify(VerifyArgs),
    /// List or delete expired receipt JSON files under a directory
    Prune(PruneArgs),
}

#[derive(Args)]
struct NewArgs {
    #[arg(long, value_name = "FILE")]
    out: PathBuf,
    #[arg(long)]
    actor: String,
    #[arg(long)]
    authorization: String,
    #[arg(long)]
    summary: String,
    #[arg(long, required = true, action = clap::ArgAction::Append)]
    scope: Vec<String>,
    #[arg(long, default_value_t = 30, value_parser = clap::value_parser!(u32).range(1..=3650))]
    retention_days: u32,
    #[arg(long, action = clap::ArgAction::Append, value_name = "ENV_NAME")]
    redact_env: Vec<String>,
    #[arg(
        long,
        value_name = "FILE",
        help = "Key destination; defaults to <receipt>.key"
    )]
    key: Option<PathBuf>,
}

#[derive(Args)]
struct RecordArgs {
    #[arg(long)]
    receipt: PathBuf,
    #[arg(long, default_value = "tool")]
    kind: String,
    #[arg(long)]
    tool: String,
    #[arg(long, default_value = "null", value_name = "JSON")]
    input_json: String,
    #[arg(long, default_value = "null", value_name = "JSON")]
    output_json: String,
    #[arg(long)]
    exit_code: Option<i32>,
    #[arg(long, action = clap::ArgAction::Append, value_name = "FILE")]
    artifact: Vec<PathBuf>,
    #[arg(long, action = clap::ArgAction::Append, value_name = "LITERAL")]
    redact: Vec<String>,
}

#[derive(Args)]
struct RunArgs {
    #[arg(long)]
    receipt: PathBuf,
    #[arg(long, default_value = "process")]
    tool: String,
    #[arg(long, action = clap::ArgAction::Append, value_name = "FILE")]
    artifact: Vec<PathBuf>,
    #[arg(long, action = clap::ArgAction::Append, value_name = "LITERAL")]
    redact: Vec<String>,
    #[arg(last = true, required = true, num_args = 1.., value_name = "COMMAND")]
    command: Vec<String>,
}

#[derive(Args)]
struct SealArgs {
    #[arg(long)]
    receipt: PathBuf,
    #[arg(long, value_name = "FILE")]
    key: Option<PathBuf>,
    #[arg(long, value_name = "FILE")]
    html: Option<PathBuf>,
}

#[derive(Args)]
struct VerifyArgs {
    #[arg(value_name = "RECEIPT_JSON_OR_HTML")]
    path: PathBuf,
    #[arg(long)]
    json: bool,
}

#[derive(Args)]
struct PruneArgs {
    #[arg(long)]
    dir: PathBuf,
    #[arg(long, value_parser = clap::value_parser!(i64).range(1..=3650))]
    older_than: i64,
    #[arg(long, help = "List matching receipts without deleting them")]
    dry_run: bool,
    #[arg(
        long,
        help = "Required to delete matching receipts and their per-receipt keys"
    )]
    confirm: bool,
}

fn main() {
    let code = match execute(Cli::parse()) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("error: {message}");
            1
        }
    };
    process::exit(code);
}

fn execute(cli: Cli) -> Result<i32, String> {
    match cli.command {
        Commands::New(args) => command_new(args),
        Commands::Record(args) => command_record(args),
        Commands::Run(args) => command_run(args),
        Commands::Seal(args) => command_seal(args),
        Commands::Verify(args) => command_verify(args),
        Commands::Prune(args) => command_prune(args),
    }
}

fn command_new(args: NewArgs) -> Result<i32, String> {
    if args.out.exists() {
        return Err(format!(
            "{} already exists; refusing to overwrite",
            args.out.display()
        ));
    }
    let key_path = args.key.unwrap_or_else(|| default_key_path(&args.out));
    if key_path.exists() {
        return Err(format!(
            "{} already exists; refusing to overwrite",
            key_path.display()
        ));
    }
    let receipt = new_receipt(
        Subject {
            summary: args.summary,
            actor: args.actor,
            authorization: args.authorization,
            scope: args.scope,
        },
        Policy {
            retention_days: args.retention_days,
            default_sensitive_keys: true,
            redact_environment: args.redact_env,
        },
    );
    write_receipt(&args.out, &receipt)?;
    write_key(&key_path, &generate_key())?;
    println!("Created open receipt {}", args.out.display());
    println!("Protected signing key {}", key_path.display());
    println!(
        "Next: action-receipts run --receipt {} -- <command>",
        args.out.display()
    );
    Ok(0)
}

fn parse_json(label: &str, text: &str) -> Result<Value, String> {
    serde_json::from_str(text).map_err(|e| format!("{label} is not valid JSON: {e}"))
}

fn command_record(args: RecordArgs) -> Result<i32, String> {
    let mut receipt = read_receipt(&args.receipt)?;
    let artifacts = args
        .artifact
        .iter()
        .map(|p| artifact_for(p))
        .collect::<Result<Vec<_>, _>>()?;
    append_event(
        &mut receipt,
        args.kind,
        args.tool,
        None,
        parse_json("--input-json", &args.input_json)?,
        parse_json("--output-json", &args.output_json)?,
        args.exit_code,
        artifacts,
        &args.redact,
    )?;
    write_receipt(&args.receipt, &receipt)?;
    println!(
        "Recorded event {} · {}",
        receipt.events.len(),
        receipt.chain_head
    );
    Ok(0)
}

fn command_run(args: RunArgs) -> Result<i32, String> {
    let mut receipt = read_receipt(&args.receipt)?;
    if receipt.state != ReceiptState::Open {
        return Err("receipt is sealed; start a new receipt".into());
    }
    let start = Instant::now();
    let output = Command::new(&args.command[0])
        .args(&args.command[1..])
        .output();
    let elapsed = start.elapsed().as_millis();
    let (exit_code, stdout, stderr, spawn_error) = match output {
        Ok(output) => (
            output.status.code().unwrap_or(1),
            capture(&output.stdout),
            capture(&output.stderr),
            Value::Null,
        ),
        Err(error) => (
            127,
            json!({"text":"", "truncated":false}),
            json!({"text":"", "truncated":false}),
            Value::String(error.to_string()),
        ),
    };
    let artifacts = args
        .artifact
        .iter()
        .map(|p| artifact_for(p))
        .collect::<Result<Vec<_>, _>>()?;
    let cwd = std::env::current_dir()
        .map_err(|e| format!("could not determine working directory: {e}"))?;
    append_event(
        &mut receipt,
        "command".into(),
        args.tool,
        Some(args.command.clone()),
        json!({"cwd": cwd}),
        json!({"stdout": stdout, "stderr": stderr, "duration_ms": elapsed, "spawn_error": spawn_error}),
        Some(exit_code),
        artifacts,
        &args.redact,
    )?;
    write_receipt(&args.receipt, &receipt)?;
    let out_text = receipt
        .events
        .last()
        .and_then(|e| e.output.get("stdout"))
        .and_then(|v| v.get("text"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let err_text = receipt
        .events
        .last()
        .and_then(|e| e.output.get("stderr"))
        .and_then(|v| v.get("text"))
        .and_then(Value::as_str)
        .unwrap_or("");
    print!("{out_text}");
    eprint!("{err_text}");
    eprintln!(
        "[action-receipts] recorded event {} with exit code {exit_code}",
        receipt.events.len()
    );
    Ok(exit_code.clamp(0, 255))
}

fn capture(bytes: &[u8]) -> Value {
    let truncated = bytes.len() > MAX_CAPTURE_BYTES;
    let slice = &bytes[..bytes.len().min(MAX_CAPTURE_BYTES)];
    json!({"text": String::from_utf8_lossy(slice), "truncated": truncated})
}

fn command_seal(args: SealArgs) -> Result<i32, String> {
    let mut receipt = read_receipt(&args.receipt)?;
    let key_path = args.key.unwrap_or_else(|| default_key_path(&args.receipt));
    let key: KeyFile = serde_json::from_str(
        &fs::read_to_string(&key_path)
            .map_err(|e| format!("could not read key {}: {e}", key_path.display()))?,
    )
    .map_err(|e| format!("invalid key file: {e}"))?;
    seal(&mut receipt, &key)?;
    write_receipt(&args.receipt, &receipt)?;
    if let Some(path) = args.html {
        fs::write(&path, html_report(&receipt)?)
            .map_err(|e| format!("could not write {}: {e}", path.display()))?;
        println!("Wrote readable receipt {}", path.display());
    }
    println!(
        "Sealed {} · {} events",
        args.receipt.display(),
        receipt.events.len()
    );
    Ok(0)
}

fn command_verify(args: VerifyArgs) -> Result<i32, String> {
    let receipt = read_receipt(&args.path)?;
    let result = verify(&receipt);
    if args.json {
        println!(
            "{}",
            serde_json::to_string(&result).map_err(|e| e.to_string())?
        );
    } else if result.valid {
        println!(
            "VALID  {}",
            result.receipt_id.as_deref().unwrap_or("unknown")
        );
        println!(
            "       {} events · chain and Ed25519 signature verified offline",
            result.event_count
        );
        println!("NOTICE Signatures prove bundle integrity, not identity, intent, or correctness.");
    } else {
        println!(
            "INVALID {}",
            result.receipt_id.as_deref().unwrap_or("unknown")
        );
        println!("        {}", result.message);
    }
    Ok(if result.valid { 0 } else { 3 })
}

fn command_prune(args: PruneArgs) -> Result<i32, String> {
    if !args.dry_run && !args.confirm {
        return Err(
            "refusing to delete without --confirm; use --dry-run to inspect matches".into(),
        );
    }
    let mut files = Vec::new();
    collect_receipts(&args.dir, &mut files)?;
    let mut matched = 0usize;
    for path in files {
        let receipt = match read_receipt(&path) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("skip {}: {e}", path.display());
                continue;
            }
        };
        if older_than(&receipt, args.older_than)? {
            matched += 1;
            if args.dry_run {
                println!("would delete {}", path.display());
            } else {
                fs::remove_file(&path)
                    .map_err(|e| format!("could not delete {}: {e}", path.display()))?;
                let key = default_key_path(&path);
                if key.exists() {
                    fs::remove_file(&key)
                        .map_err(|e| format!("could not delete {}: {e}", key.display()))?;
                }
                println!("deleted {}", path.display());
            }
        }
    }
    println!("{} expired receipt(s) matched", matched);
    Ok(0)
}

fn collect_receipts(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| format!("could not read {}: {e}", dir.display()))? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.is_dir() {
            collect_receipts(&path, out)?;
        } else if path
            .file_name()
            .and_then(|v| v.to_str())
            .is_some_and(|v| v.ends_with(".receipt.json"))
        {
            out.push(path);
        }
    }
    Ok(())
}
