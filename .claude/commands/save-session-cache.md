# Cache Project Context

I'll analyze and remember key information about this Rust project to speed up future commands.

Let me examine your project structure and save what I discover:

```powershell
# Create cache directory if needed
$CACHE_DIR = "$env:USERPROFILE\.claude\.ollama-ai-agents-playground_cache"
New-Item -ItemType Directory -Path $CACHE_DIR -Force | Out-Null

# Generate cache file name based on project path
$currentPath = (Get-Location).Path
$PROJECT_HASH = (([System.Security.Cryptography.MD5]::Create().ComputeHash([System.Text.Encoding]::UTF8.GetBytes($currentPath)) | ForEach-Object { $_.ToString('x2') }) -join '').Substring(0,8)
$CACHE_FILE = "$CACHE_DIR\project_$PROJECT_HASH.cache"
```

I'll detect and remember:
- Cargo build configuration and dependencies
- How to run tests (cargo test)
- What formatter is configured (rustfmt)
- Project structure patterns (src/, spec/ directories)
- Ollama API integration patterns
- Agent implementation patterns
- To write development notes and progress in `DEV_NOTES.md` to keep track of what I've done

This helps other commands run faster by not re-analyzing the same information repeatedly.

The cache expires after 24 hours or when key project files (Cargo.toml, config.toml, spec files) change.