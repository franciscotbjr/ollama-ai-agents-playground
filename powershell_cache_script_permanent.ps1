# PowerShell Cache Script - VERSÃO CACHE PERMANENTE
# Este script cria cache persistente por período indefinido
# O cache permanece válido até ser explicitamente removido ou substituído

# Criação do diretório cache
$CACHE_DIR = "$env:USERPROFILE\.claude\.ollama-ai-agents-playground_cache"
New-Item -ItemType Directory -Path $CACHE_DIR -Force | Out-Null

# Cálculo do hash do projeto
$currentPath = (Get-Location).Path
$bytes = [System.Text.Encoding]::UTF8.GetBytes($currentPath)
$hash = [System.Security.Cryptography.MD5]::Create().ComputeHash($bytes)
$PROJECT_HASH = ($hash | ForEach-Object { $_.ToString("x2") }) -join ""
$PROJECT_HASH = $PROJECT_HASH.Substring(0,8)
$CACHE_FILE = "$CACHE_DIR\project_$PROJECT_HASH.cache"

# Verificar se já existe cache e preservar dados importantes
$existingContext = @{}
if (Test-Path $CACHE_FILE) {
    try {
        $existingContent = Get-Content $CACHE_FILE -Raw | ConvertFrom-Json
        $existingContext = @{
            "createdAt" = $existingContent.createdAt
            "sessionCount" = if ($existingContent.sessionCount) { $existingContent.sessionCount + 1 } else { 1 }
            "firstSession" = $existingContent.firstSession
            "totalSessions" = if ($existingContent.totalSessions) { $existingContent.totalSessions + 1 } else { 1 }
        }
        Write-Host "📋 Existing cache found - updating session #$($existingContext.sessionCount)"
    } catch {
        Write-Host "⚠️  Could not read existing cache, creating new one"
        $existingContext = @{}
    }
} else {
    Write-Host "🆕 Creating new permanent cache"
}

# Criação do contexto do projeto com informações permanentes
$projectContext = @{
    # === INFORMAÇÕES PERMANENTES (nunca expiram) ===
    "cacheType" = "permanent"
    "cacheVersion" = "2.0"
    "createdAt" = if ($existingContext.createdAt) { $existingContext.createdAt } else { Get-Date -Format "yyyy-MM-dd HH:mm:ss" }
    "firstSession" = if ($existingContext.firstSession) { $existingContext.firstSession } else { Get-Date -Format "yyyy-MM-dd HH:mm:ss" }
    "expirationPolicy" = "never"
    "persistenceLevel" = "indefinite"
    
    # === INFORMAÇÕES DE SESSÃO (atualizadas a cada execução) ===
    "lastSession" = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    "sessionCount" = if ($existingContext.sessionCount) { $existingContext.sessionCount } else { 1 }
    "totalSessions" = if ($existingContext.totalSessions) { $existingContext.totalSessions } else { 1 }
    
    # === INFORMAÇÕES DO PROJETO (permanentes) ===
    "projectPath" = $currentPath
    "projectHash" = $PROJECT_HASH
    "buildSystem" = "cargo"
    "testFramework" = "cargo test (Rust built-in)"
    "formatter" = "rustfmt"
    "language" = "Rust 2024"
    "ollamaModel" = "gemma3"
    
    # === STATUS ATUAL DO PROJETO ===
    "currentBranch" = "feature/create_assistant_agent"
    "mainBranch" = "main"
    "totalTests" = 143
    "buildStatus" = "passing"
    "intentCount" = 17
    
    # === FUNCIONALIDADES ENHANCED ===
    "enhancedFeatures" = @("Quick research", "Word assistance", "Unit conversion", "Math calculations")
    "recentWork" = "Enhanced Personal Assistant with 17 intents, translated to English"
    
    # === ARQUIVOS E ESTRUTURA (permanentes) ===
    "keyFiles" = @("Cargo.toml", "config.toml", "DEV_NOTES.md", "README.md", "spec/PERSONAL_ASSISTANT_EXAMPLE.md")
    "coreDirectories" = @("src/agent/", "src/infra/", "src/assistant/", "spec/")
    
    # === HISTÓRICO E APRENDIZADO (cumulativo) ===
    "knownPatterns" = @{
        "agentSystem" = "Generic Agent<T> trait with modular result types"
        "ollamaIntegration" = "Gemma3 model at localhost:11434"
        "intentClassification" = "17 specialized intents with parameter extraction"
        "testingApproach" = "cargo test with 143 tests"
    }
    
    # === CONFIGURAÇÕES DE DESENVOLVIMENTO ===
    "devEnvironment" = @{
        "os" = "Windows"
        "shell" = "bash + PowerShell"
        "cacheLocation" = $CACHE_DIR
        "workingDirectory" = $currentPath
    }
    
    # === METADADOS DO CACHE ===
    "cacheMetadata" = @{
        "lastUpdate" = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        "updateReason" = "Permanent cache update - session continuation"
        "dataIntegrity" = "validated"
        "retentionPolicy" = "indefinite"
    }
}

# Salvamento do contexto em JSON com formatação melhorada
$json = $projectContext | ConvertTo-Json -Depth 4
$json | Out-File -FilePath $CACHE_FILE -Encoding UTF8

# Verificação e relatório detalhado
Write-Host "💾 Permanent cache location: $CACHE_FILE"
Write-Host "🔢 Project hash: $PROJECT_HASH"
Write-Host "📊 Context saved with $(($projectContext.Keys).Count) properties"
Write-Host "🔄 Session #$($projectContext.sessionCount) (Total: $($projectContext.totalSessions))"
Write-Host "🕐 Created: $($projectContext.createdAt)"
Write-Host "♾️  Expiration: NEVER (Permanent cache)"

if (Test-Path $CACHE_FILE) {
    $fileSize = (Get-Item $CACHE_FILE).Length
    Write-Host "✅ Permanent cache saved successfully ($fileSize bytes)"
    Write-Host "🛡️  Cache will persist indefinitely until manually removed"
} else {
    Write-Host "❌ Error: Cache file not created"
}

# Adicionar informações sobre como remover o cache se necessário
Write-Host ""
Write-Host "To manually remove cache: Remove-Item '$CACHE_FILE'"
Write-Host "Cache directory: $CACHE_DIR"