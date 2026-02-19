# Conventions Skill — ollama-ai-agents-playground

Padrões de código Rust para este projeto.

---

## Nomenclatura

| Item | Estilo | Exemplo |
|------|--------|---------|
| Tipos (struct, enum, trait) | PascalCase | `ClassificationResult`, `AgentError` |
| Funções e métodos | snake_case | `process()`, `build_assistant_name()` |
| Constantes | SCREAMING_SNAKE_CASE | `PERSONAL_IDENTITY`, `DEFAULT_URL` |
| Módulos e arquivos | snake_case | `intent_classifier_agent.rs` |
| Variáveis | snake_case | `user_input`, `check_result` |
| Variantes de erro | `{Type}Error` sufixo | `ProcessingError`, `NetworkError` |

---

## Um tipo por arquivo

Cada arquivo `.rs` contém exatamente **um tipo primário**. Tipos auxiliares pequenos (ex: função `build_assistant_name`) podem coexistir, mas não outros structs ou enums independentes.

```
✅ classification_result.rs  →  struct ClassificationResult
✅ intent_param.rs           →  struct IntentParam
✅ mapper_error.rs           →  enum MapperError
✅ mapper.rs                 →  trait Mapper<T, U>

❌ classification_result.rs  →  ClassificationResult + OllamaIntentResponseParser
❌ check_assistant_agent.rs  →  CheckAssistantAgent + CheckParam
```

---

## mod.rs como Fachada

`mod.rs` é exclusivamente para declarações de submódulos e re-exports. Sem lógica:

```rust
// ✅ Correto
pub mod classification_result;
pub mod intent_param;

pub use classification_result::ClassificationResult;
pub use intent_param::IntentParam;

// ❌ Errado — lógica no mod.rs
pub fn build_something() { ... }
```

---

## Imports

Organizar em grupos separados por linha em branco:

```rust
// 1. Biblioteca padrão
use std::error::Error;
use std::fmt;

// 2. Dependências externas
use serde::{Deserialize, Serialize};
use ollama_oxide::ResponseMessage;

// 3. Crate atual (crate::)
use crate::agents::{AgentResult, Intent};
use crate::config::Config;
```

---

## Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub intent: Intent,
    pub params: Params,
}
```

Ordem dos derives: `Debug`, `Clone`, `Copy` (se aplicável), `Serialize`, `Deserialize`, `Default` (se aplicável).

Campos opcionais:
```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub recipient: Option<String>,
```

---

## Enums

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    SendEmail,
    ScheduleMeeting,
    NoAction,
}
```

---

## Erros

Todos os enums de erro usam sufixo `{Type}Error` nas variantes:

```rust
#[derive(Debug)]
pub enum AgentError {
    ProcessingError(String),   // ✅
    NetworkError(String),      // ✅
    ParseError(String),        // ✅
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::ProcessingError(msg) => write!(f, "Processing error: {msg}"),
            AgentError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            AgentError::ParseError(msg) => write!(f, "Parse error: {msg}"),
        }
    }
}

impl std::error::Error for AgentError {}
```

---

## Construtores e Métodos

Preferir o padrão `new()` simples:

```rust
impl CheckParam {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
```

Para tipos com muitos campos opcionais, usar with-method chain:

```rust
impl Params {
    pub fn new() -> Self { Self { recipient: None, message: None } }
    pub fn with_recipient(mut self, r: String) -> Self { self.recipient = Some(r); self }
    pub fn with_message(mut self, m: String) -> Self { self.message = Some(m); self }
}
```

---

## Visibilidade

- Padrão: privado
- `pub` para tipos e métodos da API pública do módulo
- `pub(super)` para helpers internos ao módulo pai
- `pub(crate)` para utilitários internos à crate

---

## Async

Todos os agentes são async via trait:

```rust
fn process(&self, input: P)
    -> impl Future<Output = Result<T, AgentError>> + Send;
```

Testes async usam `#[tokio::test]`:

```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_something() { ... }
}
```

---

## Serde

```rust
// Enum com snake_case na serialização
#[serde(rename_all = "snake_case")]
pub enum Intent { SendEmail, ScheduleMeeting }

// Campo opcional omitido quando None
#[serde(skip_serializing_if = "Option::is_none")]
pub field: Option<String>,

// Campo com valor padrão
#[serde(default)]
pub flag: bool,
```
