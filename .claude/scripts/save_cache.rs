#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! chrono = "0.4"
//! toml = "0.8"
//! ```

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

const MAX_SESSIONS: usize = 10;
const MAX_TASK_LEN: usize = 200;
const MAX_SUMMARY_LEN: usize = 500;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SessionEntry {
    datetime: String,
    task: String,
    summary: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectContext {
    // Project identification (from Cargo.toml)
    project_name: String,
    version: String,
    repository: String,
    license: String,

    // Build system
    build_system: String,
    language: String,
    edition: String,

    // Workspace structure (from Cargo.toml)
    workspace_crates: Vec<String>,
    total_crates: u32,

    // Critical files inventory
    critical_files: Vec<String>,
    apis_spec_files: Vec<String>,
    impl_files: Vec<String>,

    // Session tracking
    session_count: u32,
    total_sessions: u32,
    created_at: String,
    last_session: String,
    project_path: String,

    // Build status (check if compilable)
    build_status: String,

    // Metadata
    cache_version: String,
    project_hash: String,

    // Session context: array of last 10 sessions ordered by datetime
    #[serde(default)]
    session_context: Vec<SessionEntry>,
}

#[derive(Deserialize)]
struct CargoToml {
    package: Option<Package>,
}

#[derive(Deserialize)]
struct Package {
    name: Option<String>,
    version: Option<String>,
    repository: Option<String>,
    license: Option<String>,
    edition: Option<String>,
}

fn get_cache_dir() -> PathBuf {
    let home = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .expect("Could not find home directory");
    PathBuf::from(home).join(".claude").join("ollama-ai-agents-playground")
}

fn get_project_hash() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let current_dir = env::current_dir()
        .expect("Could not get current directory")
        .to_string_lossy()
        .to_string();

    let mut hasher = DefaultHasher::new();
    current_dir.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:08x}", hash)
}

fn get_cache_file(cache_dir: &PathBuf) -> PathBuf {
    cache_dir.join("project.cache")
}

fn get_backup_file(cache_dir: &PathBuf) -> PathBuf {
    cache_dir.join("project.cache.bkp")
}

fn load_existing_cache(cache_file: &PathBuf) -> Option<ProjectContext> {
    if cache_file.exists() {
        if let Ok(content) = fs::read_to_string(cache_file) {
            if let Ok(context) = serde_json::from_str::<ProjectContext>(&content) {
                println!(
                    "Found existing cache (Session #{}, {} previous sessions recorded)",
                    context.total_sessions + 1,
                    context.session_context.len()
                );
                return Some(context);
            }
        }
    }
    println!("Creating new cache file");
    None
}

fn create_backup(cache_dir: &PathBuf) {
    let cache_file = get_cache_file(cache_dir);
    let backup_file = get_backup_file(cache_dir);

    if cache_file.exists() {
        match fs::copy(&cache_file, &backup_file) {
            Ok(_) => println!("📦 Backup created: {}", backup_file.display()),
            Err(e) => println!("⚠️  Failed to create backup: {}", e),
        }
    }
}

fn read_cargo_toml() -> Result<CargoToml, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("Cargo.toml")?;
    Ok(toml::from_str(&content)?)
}

fn find_critical_files() -> Vec<String> {
    let mut files = Vec::new();
    let critical = [
        "spec/definition.md",
        "spec/api-analysis.md",
        "DEV_NOTES.md",
        "CHANGELOG.md",
        "README.md",
        "CONTRIBUTING.md",
        "ARCHITECTURE.md",
        "DECISIONS.md",
        "BLOCKERS.md",
        "Cargo.toml",
    ];

    for file in critical {
        if PathBuf::from(file).exists() {
            files.push(file.to_string());
        }
    }
    files
}

fn find_apis_spec_files() -> Vec<String> {
    let mut files = Vec::new();
    let spec_dir = PathBuf::from("spec/apis");

    if spec_dir.exists() {
        if let Ok(entries) = fs::read_dir(&spec_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".yaml") {
                        files.push(format!("spec/apis/{}", name));
                    }
                }
            }
        }
    }
    files.sort();
    files
}

fn find_impl_files() -> Vec<String> {
    let mut files = Vec::new();
    let impl_dir = PathBuf::from("impl");

    if impl_dir.exists() {
        if let Ok(entries) = fs::read_dir(&impl_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".md") {
                        files.push(format!("impl/{}", name));
                    }
                }
            }
        }
    }
    files.sort();
    files
}

fn check_build_status() -> String {
    if PathBuf::from("Cargo.toml").exists() {
        match read_cargo_toml() {
            Ok(_) => "Cargo.toml valid".to_string(),
            Err(_) => "Cargo.toml has errors".to_string(),
        }
    } else {
        "No Cargo.toml found".to_string()
    }
}

fn parse_cli_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    let mut task = String::new();
    let mut summary = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--task" => {
                if i + 1 < args.len() {
                    task = args[i + 1].clone();
                    i += 1;
                }
            }
            "--summary" => {
                if i + 1 < args.len() {
                    summary = args[i + 1].clone();
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    (task, summary)
}

/// Sanitize text for safe JSON storage: remove control characters and truncate.
fn sanitize_text(text: &str, max_len: usize) -> String {
    // Replace control characters (except space) with spaces
    let cleaned: String = text
        .chars()
        .map(|c| if c.is_control() && c != ' ' { ' ' } else { c })
        .collect();

    // Collapse multiple whitespace into single space
    let collapsed: String = cleaned.split_whitespace().collect::<Vec<&str>>().join(" ");

    // Truncate at max_len chars (not bytes) on a word boundary
    if collapsed.chars().count() <= max_len {
        collapsed
    } else {
        let truncated: String = collapsed.chars().take(max_len).collect();
        // Try to cut at last space to avoid mid-word truncation
        match truncated.rfind(' ') {
            Some(pos) if pos > truncated.len() / 2 => format!("{}...", &truncated[..pos]),
            _ => format!("{}...", truncated),
        }
    }
}

fn cleanup_old_cache_files(cache_dir: &PathBuf) {
    // Remove old hash-based cache files (project_{hash}.cache)
    if let Ok(entries) = fs::read_dir(cache_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("project_") && name.ends_with(".cache") && name != "project.cache" {
                    match fs::remove_file(entry.path()) {
                        Ok(_) => println!("🧹 Removed old cache file: {}", name),
                        Err(e) => println!("⚠️  Failed to remove old cache file {}: {}", name, e),
                    }
                }
            }
        }
    }
}

fn main() {
    let cache_dir = get_cache_dir();
    let project_hash = get_project_hash();
    let cache_file = get_cache_file(&cache_dir);

    // Create cache directory if needed
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");
        println!("Created cache directory: {}", cache_dir.display());
    }

    // Load existing cache or start fresh
    let existing = load_existing_cache(&cache_file);
    let existing_sessions = existing.as_ref().map(|ctx| ctx.total_sessions + 1).unwrap_or(1);
    let mut previous_session_entries: Vec<SessionEntry> = existing
        .as_ref()
        .map(|ctx| ctx.session_context.clone())
        .unwrap_or_default();

    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let created_at = existing
        .as_ref()
        .map(|ctx| ctx.created_at.clone())
        .unwrap_or_else(|| current_time.clone());

    let current_dir = env::current_dir()
        .expect("Could not get current directory")
        .to_string_lossy()
        .to_string();

    // Read project info from Cargo.toml
    let cargo_toml = read_cargo_toml().expect("Failed to read Cargo.toml");

    let project_name = cargo_toml.package.as_ref()
        .and_then(|p| p.name.clone())
        .unwrap_or_else(|| "ollama-ai-agents-playground".to_string());

    let version = cargo_toml.package.as_ref()
        .and_then(|p| p.version.clone())
        .unwrap_or_else(|| "0.1.0".to_string());

    let repository = cargo_toml.package.as_ref()
        .and_then(|p| p.repository.clone())
        .unwrap_or_else(|| "https://github.com/franciscotbjr/ollama-ai-agents-playground".to_string());

    let license = cargo_toml.package.as_ref()
        .and_then(|p| p.license.clone())
        .unwrap_or_else(|| "MIT".to_string());

    let edition = cargo_toml.package.as_ref()
        .and_then(|p| p.edition.clone())
        .unwrap_or_else(|| "2024".to_string());

    // Single crate - no workspace
    let workspace_crates = vec![project_name.clone()];
    let total_crates = 1;

    // Find critical, spec, and impl files
    let critical_files = find_critical_files();
    let apis_spec_files = find_apis_spec_files();
    let impl_files = find_impl_files();

    // Parse CLI args for session context
    let (task, summary) = parse_cli_args();

    // Sanitize and truncate to prevent cache corruption from oversized input
    let task = sanitize_text(&task, MAX_TASK_LEN);
    let summary = sanitize_text(&summary, MAX_SUMMARY_LEN);

    // Add new session entry to the array
    let new_entry = SessionEntry {
        datetime: current_time.clone(),
        task,
        summary,
    };
    previous_session_entries.push(new_entry);

    // Keep only the last MAX_SESSIONS entries, ordered by datetime
    previous_session_entries.sort_by(|a, b| a.datetime.cmp(&b.datetime));
    if previous_session_entries.len() > MAX_SESSIONS {
        let start = previous_session_entries.len() - MAX_SESSIONS;
        previous_session_entries = previous_session_entries[start..].to_vec();
    }

    // Build context object
    let context = ProjectContext {
        project_name: project_name.clone(),
        version: version.clone(),
        repository,
        license,
        build_system: "Cargo (workspace)".to_string(),
        language: "Rust".to_string(),
        edition,
        workspace_crates: workspace_crates.clone(),
        total_crates,
        critical_files,
        apis_spec_files: apis_spec_files.clone(),
        impl_files: impl_files.clone(),
        session_count: existing_sessions,
        total_sessions: existing_sessions,
        created_at,
        last_session: current_time,
        project_path: current_dir,
        build_status: check_build_status(),
        cache_version: "2.0".to_string(),
        project_hash: project_hash.clone(),
        session_context: previous_session_entries,
    };

    // Save to cache file
    let json = serde_json::to_string_pretty(&context).expect("Failed to serialize context");
    fs::write(&cache_file, &json).expect("Failed to write cache file");

    // Create backup after successful write
    create_backup(&cache_dir);

    // Clean up old hash-based cache files
    cleanup_old_cache_files(&cache_dir);

    println!("\n✅ Context saved successfully!\n");
    println!("📊 Cache Summary:");
    println!("  Location: {}", cache_file.display());
    println!("  Backup: {}", get_backup_file(&cache_dir).display());
    println!("  Project: {} v{}", context.project_name, context.version);
    println!("  Session: #{}", context.session_count);
    println!("  Architecture: Single crate with {} modules", context.total_crates);
    println!("  API Specs: {} endpoints", apis_spec_files.len());
    println!("  Impl Plans: {} files", impl_files.len());
    println!("  Build: {}", context.build_status);
    println!("  Sessions Recorded: {}", context.session_context.len());
    println!("\n📁 Critical Files Tracked:");
    for file in &context.critical_files {
        println!("  ✓ {}", file);
    }

    // Display session history
    if !context.session_context.is_empty() {
        println!("\n📝 Session History (last {}):", context.session_context.len());
        for (i, entry) in context.session_context.iter().enumerate() {
            let task_display = if entry.task.is_empty() { "(no task)" } else { &entry.task };
            println!("  {}. [{}] {}", i + 1, entry.datetime, task_display);
            if !entry.summary.is_empty() {
                println!("     Summary: {}", entry.summary);
            }
        }
    }

    println!("\nReady to continue in next session with /continue-session");
}
