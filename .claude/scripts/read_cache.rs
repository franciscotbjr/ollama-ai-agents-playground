#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct SessionEntry {
    datetime: String,
    task: String,
    summary: String,
}

// Legacy format support (v1.x)
#[derive(Serialize, Deserialize, Debug, Default)]
struct LegacySessionContext {
    task: String,
    summary: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectContext {
    project_name: String,
    version: String,
    repository: String,
    license: String,
    build_system: String,
    language: String,
    edition: String,
    workspace_crates: Vec<String>,
    total_crates: u32,
    critical_files: Vec<String>,
    apis_spec_files: Vec<String>,
    #[serde(default)]
    impl_files: Vec<String>,
    session_count: u32,
    total_sessions: u32,
    created_at: String,
    last_session: String,
    project_path: String,
    build_status: String,
    cache_version: String,
    project_hash: String,
    // v2.0: array of session entries
    #[serde(default)]
    session_context: Vec<SessionEntry>,
}

// Legacy format for migration
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct LegacyProjectContext {
    project_name: String,
    version: String,
    repository: String,
    license: String,
    build_system: String,
    language: String,
    edition: String,
    workspace_crates: Vec<String>,
    total_crates: u32,
    critical_files: Vec<String>,
    apis_spec_files: Vec<String>,
    #[serde(default)]
    impl_files: Vec<String>,
    session_count: u32,
    total_sessions: u32,
    created_at: String,
    last_session: String,
    project_path: String,
    build_status: String,
    cache_version: String,
    project_hash: String,
    #[serde(default)]
    session_context: LegacySessionContext,
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

/// Try to find cache: first project.cache, then legacy project_{hash}.cache, then backup
fn find_cache_file(cache_dir: &PathBuf, project_hash: &str) -> Option<PathBuf> {
    // 1. Try new unified file
    let unified = get_cache_file(cache_dir);
    if unified.exists() {
        return Some(unified);
    }

    // 2. Try legacy hash-based file
    let legacy = cache_dir.join(format!("project_{}.cache", project_hash));
    if legacy.exists() {
        println!("  (migrating from legacy cache format)");
        return Some(legacy);
    }

    // 3. Try backup file as last resort
    let backup = get_backup_file(cache_dir);
    if backup.exists() {
        println!("  (restoring from backup)");
        return Some(backup);
    }

    None
}

/// Parse cache content, handling both v1.x (legacy) and v2.0 formats
fn parse_cache(content: &str) -> Result<ProjectContext, String> {
    // Try v2.0 format first (session_context is Vec<SessionEntry>)
    if let Ok(context) = serde_json::from_str::<ProjectContext>(content) {
        return Ok(context);
    }

    // Try legacy format (session_context is {task, summary})
    if let Ok(legacy) = serde_json::from_str::<LegacyProjectContext>(content) {
        // Migrate legacy session_context to new format
        let mut sessions = Vec::new();
        if !legacy.session_context.task.is_empty() || !legacy.session_context.summary.is_empty() {
            sessions.push(SessionEntry {
                datetime: legacy.last_session.clone(),
                task: legacy.session_context.task,
                summary: legacy.session_context.summary,
            });
        }

        return Ok(ProjectContext {
            project_name: legacy.project_name,
            version: legacy.version,
            repository: legacy.repository,
            license: legacy.license,
            build_system: legacy.build_system,
            language: legacy.language,
            edition: legacy.edition,
            workspace_crates: legacy.workspace_crates,
            total_crates: legacy.total_crates,
            critical_files: legacy.critical_files,
            apis_spec_files: legacy.apis_spec_files,
            impl_files: legacy.impl_files,
            session_count: legacy.session_count,
            total_sessions: legacy.total_sessions,
            created_at: legacy.created_at,
            last_session: legacy.last_session,
            project_path: legacy.project_path,
            build_status: legacy.build_status,
            cache_version: "2.0".to_string(),
            project_hash: legacy.project_hash,
            session_context: sessions,
        });
    }

    Err("Failed to parse cache file in any known format".to_string())
}

fn read_file_summary(file_path: &str) -> String {
    if let Ok(content) = fs::read_to_string(file_path) {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        let first_line = lines.iter()
            .find(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .map(|s| s.trim())
            .unwrap_or("(empty file)");

        if total_lines > 50 {
            format!("{} lines - {}", total_lines, first_line)
        } else {
            format!("{} lines", total_lines)
        }
    } else {
        "(not readable)".to_string()
    }
}

fn display_blockers() {
    let blockers_path = "BLOCKERS.md";

    if let Ok(content) = fs::read_to_string(blockers_path) {
        let lines: Vec<&str> = content.lines().collect();

        let mut in_active_section = false;
        let mut active_blockers: Vec<&str> = Vec::new();

        for line in &lines {
            if line.contains("## Bloqueios Ativos") {
                in_active_section = true;
                continue;
            }
            if in_active_section && line.starts_with("## ") {
                break;
            }
            if in_active_section && line.starts_with('|') && !line.contains("---") && !line.contains("Date") {
                active_blockers.push(line);
            }
        }

        if !active_blockers.is_empty() {
            println!("🚧 Active Blockers ({}):", active_blockers.len());
            for row in &active_blockers {
                let cols: Vec<&str> = row.split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                if cols.len() >= 3 {
                    let blocker_type = cols.get(1).unwrap_or(&"");
                    let blocker_desc = cols.get(2).unwrap_or(&"");
                    println!("  ⚠️  [{}] {}", blocker_type, blocker_desc);
                }
            }
            println!();
        } else {
            println!("🚧 Active Blockers: None");
            println!();
        }
    }
}

fn display_next_steps() {
    let dev_notes_path = "DEV_NOTES.md";

    if let Ok(content) = fs::read_to_string(dev_notes_path) {
        let lines: Vec<&str> = content.lines().collect();

        let mut in_todo_section = false;
        let mut todo_items: Vec<&str> = Vec::new();

        for line in &lines {
            if line.contains("### TODO") {
                in_todo_section = true;
                continue;
            }
            if in_todo_section && line.starts_with("##") {
                break;
            }
            if in_todo_section && line.trim().starts_with("- [ ]") {
                let task = line.trim().trim_start_matches("- [ ]").trim();
                todo_items.push(task);
            }
        }

        if !todo_items.is_empty() {
            let show_count = std::cmp::min(5, todo_items.len());
            println!("📌 Next Steps ({} pending, showing first {}):", todo_items.len(), show_count);

            for (i, task) in todo_items.iter().take(show_count).enumerate() {
                println!("  {}. {}", i + 1, task);
            }
            println!();
        }
    }
}

fn display_decisions() {
    let decisions_path = "DECISIONS.md";

    if let Ok(content) = fs::read_to_string(decisions_path) {
        let lines: Vec<&str> = content.lines().collect();

        let table_rows: Vec<&str> = lines.iter()
            .filter(|line| line.starts_with('|') && !line.contains("---"))
            .copied()
            .collect();

        if table_rows.len() > 1 {
            let decisions: Vec<&str> = table_rows.iter()
                .skip(1)
                .copied()
                .collect();

            let recent_count = std::cmp::min(5, decisions.len());
            let recent_decisions: Vec<&str> = decisions.iter()
                .rev()
                .take(recent_count)
                .rev()
                .copied()
                .collect();

            println!("📜 Recent Decisions ({} total, showing last {}):", decisions.len(), recent_count);

            for row in recent_decisions {
                let cols: Vec<&str> = row.split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                if cols.len() >= 2 {
                    let date = cols.first().unwrap_or(&"");
                    let decision = cols.get(1).unwrap_or(&"");
                    println!("  [{:}] {}", date, decision);
                }
            }
            println!();
        }
    } else {
        println!("📜 Decisions: No DECISIONS.md found (consider creating one)");
        println!();
    }
}

fn main() {
    let cache_dir = get_cache_dir();
    let project_hash = get_project_hash();

    println!("🔍 Loading previous conversation context...\n");

    // Find cache file (unified, legacy, or backup)
    let cache_file = match find_cache_file(&cache_dir, &project_hash) {
        Some(path) => path,
        None => {
            println!("❌ No previous conversation found");
            println!("   Cache directory: {}", cache_dir.display());
            println!("\n💡 Tip: Run /save-session-cache to create a cache for this project");
            std::process::exit(1);
        }
    };

    // Read and parse cache
    let content = fs::read_to_string(&cache_file)
        .expect("Failed to read cache file");

    let context = match parse_cache(&content) {
        Ok(ctx) => ctx,
        Err(e) => {
            println!("❌ Failed to parse cache: {}", e);
            // Try backup
            let backup = get_backup_file(&cache_dir);
            if backup.exists() && backup != cache_file {
                println!("   Trying backup file...");
                let backup_content = fs::read_to_string(&backup)
                    .expect("Failed to read backup file");
                match parse_cache(&backup_content) {
                    Ok(ctx) => {
                        println!("   ✅ Restored from backup!");
                        ctx
                    }
                    Err(e2) => {
                        println!("   ❌ Backup also failed: {}", e2);
                        std::process::exit(1);
                    }
                }
            } else {
                std::process::exit(1);
            }
        }
    };

    // Display cache summary
    println!("✅ Context loaded successfully! (cache v{})\n", context.cache_version);

    // Display session history
    if !context.session_context.is_empty() {
        println!("📝 Session History (last {}):", context.session_context.len());
        for (i, entry) in context.session_context.iter().enumerate() {
            let task_display = if entry.task.is_empty() { "(no task)" } else { &entry.task };
            println!("  {}. [{}] {}", i + 1, entry.datetime, task_display);
            if !entry.summary.is_empty() {
                println!("     Summary: {}", entry.summary);
            }
        }
        println!();
    }

    println!("📊 Project Information:");
    println!("  Project: {} v{}", context.project_name, context.version);
    println!("  Language: {} (edition {})", context.language, context.edition);
    println!("  Repository: {}", context.repository);
    println!("  License: {}", context.license);
    println!();

    println!("🏗️  Architecture:");
    println!("  Type: Single crate");
    println!("  Build System: {}", context.build_system);
    println!("  Modules: inference, http, conveniences");
    println!("  Features: default (http + inference), conveniences (optional)");
    println!();

    println!("📁 Critical Files ({} tracked):", context.critical_files.len());
    for file in &context.critical_files {
        let summary = read_file_summary(file);
        println!("  ✓ {} ({})", file, summary);
    }
    println!();

    println!("📄 API Specifications ({} endpoints):", context.apis_spec_files.len());
    let mut simple = Vec::new();
    let mut medium = Vec::new();
    let mut complex = Vec::new();

    for spec in &context.apis_spec_files {
        let filename = spec.split('/').last().unwrap_or(spec);
        if filename.contains("version") || filename.contains("tags") ||
           filename.contains("ps") || filename.contains("copy") ||
           filename.contains("delete") {
            simple.push(filename);
        } else if filename.contains("show") || filename.contains("embed") {
            medium.push(filename);
        } else {
            complex.push(filename);
        }
    }

    if !simple.is_empty() {
        println!("  Simple ({}):", simple.len());
        for spec in simple {
            println!("    - {}", spec);
        }
    }
    if !medium.is_empty() {
        println!("  Medium ({}):", medium.len());
        for spec in medium {
            println!("    - {}", spec);
        }
    }
    if !complex.is_empty() {
        println!("  Complex ({}):", complex.len());
        for spec in complex {
            println!("    - {}", spec);
        }
    }
    println!();

    if !context.impl_files.is_empty() {
        println!("📝 Implementation Plans ({} files):", context.impl_files.len());
        for impl_file in &context.impl_files {
            let filename = impl_file.split('/').last().unwrap_or(impl_file);
            let summary = read_file_summary(impl_file);
            println!("  ✓ {} ({})", filename, summary);
        }
        println!();
    }

    println!("📈 Session Information:");
    println!("  Session: #{}", context.session_count);
    println!("  Total Sessions: {}", context.total_sessions);
    println!("  Sessions Recorded: {}", context.session_context.len());
    println!("  Created: {}", context.created_at);
    println!("  Last Session: {}", context.last_session);
    println!();

    println!("🔨 Build Status:");
    println!("  Status: {}", context.build_status);
    println!();

    println!("📍 Project Location:");
    println!("  Path: {}", context.project_path);
    println!("  Hash: {}", context.project_hash);
    println!();

    // Read current phase from definition.md
    if let Ok(def_content) = fs::read_to_string("spec/definition.md") {
        println!("📋 Current Implementation Phase:");

        if def_content.contains("Phase 1 (v0.1.0)") {
            println!("  Phase: 1 (v0.1.0) - Foundation + HTTP Core");
            println!("  Status: In Progress");

            if def_content.contains("**In Progress:**") {
                println!("  Focus:");
                if def_content.contains("Simple endpoints (1): version") {
                    println!("    - Implementing GET /api/version endpoint");
                }
                if def_content.contains("Primitives crate structure") {
                    println!("    - Setting up inference crate structure");
                }
                if def_content.contains("HTTP client implementation") {
                    println!("    - Building HTTP client in http-core");
                }
                if def_content.contains("Error type hierarchy") {
                    println!("    - Creating error handling system");
                }
            }
        }
        println!();
    }

    display_decisions();
    display_blockers();
    display_next_steps();

    println!("🚀 Ready to continue where we left off!");
}
