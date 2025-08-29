# Development Notes - Ollama AI Agents Playground

## Project Cache Generated: 2025-08-29

### Project Overview
- **Language**: Rust (edition 2024)
- **Name**: ollama-ai-agents-playground
- **Version**: 0.1.0

### Key Dependencies
- serde (serialization)
- reqwest (HTTP client with blocking, json, cookies)
- tokio (async runtime with full features)
- toml, serde_json (config/data parsing)
- once_cell (lazy static)

### Configuration
- **Ollama API**: http://localhost:11434/api/chat
- **Default Model**: gemma3
- **Database Path**: D:\\development\\assistant.db

### Project Structure
- `src/agent/` - Agent implementations (classifier, assistant, contact, email)
- `src/infra/` - Infrastructure (ollama client, http, contacts, email)
- `src/assistant/` - Personal assistant implementation
- `spec/` - Specifications and Postman collection
- Test data: `contacts.json`, `calendar.json`

### Build & Test Commands
- **Build**: `cargo build`
- **Test**: `cargo test`
- **Run**: `cargo run`
- **Format**: `rustfmt` (standard Rust formatter)

### Agent Architecture
- Generic `Agent<T: AgentResult>` trait
- Type-safe agent system with modular result types
- Intent classification with parameter extraction
- Supports email, contact, and assistant agents

### Current Branch
- **Feature Branch**: feature/create_assistant_agent
- **Main Branch**: main
- Recent work on agent classification and Ollama integration

### Development Workflow
- Use this DEV_NOTES.md to track progress
- Agent system is modular and extensible
- Ollama integration with reasoning control via `think` field