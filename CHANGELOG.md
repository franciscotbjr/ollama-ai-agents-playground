# Changelog

Todas as mudanças notáveis neste projeto serão documentadas aqui.

Formato baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/).

---

## [Unreleased]

### Added
- Metodologia Design Source aplicada ao projeto
- Skills do Claude em `.claude/skills/` (architecture, conventions, agent-design, implementation, testing, documentation)
- Documentação obrigatória: `ARCHITECTURE.md`, `DECISIONS.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `BLOCKERS.md`
- Reorganização de arquivos: um tipo por arquivo (Design Source)
- Testes de integração em `tests/`
- Exemplos executáveis em `examples/`
- Estrutura Design Source para `workflowrs/`

---

## [0.4.0] - 2025-09-12

### Added
- Workspace Rust com membros `ollama-ai-agents-playground` e `workflowrs`
- Crate `workflowrs` como biblioteca independente de workflow
- Migração para `ollama-oxide` v0.1.2 como cliente Ollama
- `AssistantOllamaClient` encapsulando `ollama-oxide`

### Changed
- `Cargo.toml` convertido para workspace root
- Dependências compartilhadas via `[workspace.dependencies]`

---

## [0.3.0] - 2025-09-05

### Added
- Sistema de configuração de usuário (`[user.settings]` no config.toml)
- `UserConfig` e `UserSettings` structs com suporte a Clone
- Suporte NDJSON para endpoint de criação do Ollama
- `check_model_exists()` usando endpoint `/show` do Ollama
- Configuração de endpoints separados (chat, create, show, load)
- `AssistantRootConfig::to_name()` para nomear modelos personalizados

### Changed
- `main.rs` usa `Config::get().user.settings` (sem valores hardcoded)
- `ApiConfig` expandido com helper methods (`chat_url()`, `create_url()`, etc.)

### Fixed
- "error decoding response body" no endpoint de criação (formato NDJSON)

---

## [0.2.0] - 2025-09-01

### Added
- `AgentPrompt` e `AgentPromptBuilder` para construção de prompts
- `CheckAssistantAgent` e `CreateAssistantAgent` para gestão de modelos Ollama
- `FromMarkdownJson<T>` trait para parsing de respostas em markdown
- `OllamaToClassificationMapper` e `ToClassificationResult` extension trait
- `MapperError` enum para erros de mapeamento
- `ClassificationResult`, `Params` com serialização JSON completa

### Changed
- `ClassifierPrompt` renomeado para `AgentPrompt` (mais genérico)
- `AgentPrompt` movido de `classifier/` para `agents/agent_prompt.rs`
- Intent classification expandida para suporte de 13 tipos de intenção

### Fixed
- Erros de compilação em `CreateAssistantAgent`

---

## [0.1.0] - 2025-08-30

### Added
- Projeto inicial com arquitetura de agentes AI
- Trait `Agent<P: AgentParam, T: AgentResult>` genérico
- `IntentClassifierAgent` com classificação via Ollama
- Intent enum: `SendEmail`, `ScheduleMeeting`, `NoAction`
- `AgentError` enum: `ProcessingError`, `NetworkError`, `ParseError`
- Sistema de configuração TOML com `Config::get()` (lazy static)
- Integração com Ollama usando modelo `qwen3:0.6b`
- Temperature = 0.0 para classificação determinística
- Estrutura two-message (system + user) para melhor consistência
- Stubs para `EmailAgent`, `ContactAgent`, `PersonalAssistant`
- Especificações em `spec/`
