# Architecture — ollama-ai-agents-playground

## Visão Geral

Sistema de orquestração de agentes AI em Rust que utiliza o Ollama para classificação de intenção e automação de tarefas. O projeto demonstra uma arquitetura modular e extensível para agentes AI com tipagem forte e processamento assíncrono.

---

## Arquitetura em Camadas

```
┌─────────────────────────────────────────────┐
│              src/main.rs                    │
│         Aplicação / Demo                    │
└───────────────────┬─────────────────────────┘
                    │
┌───────────────────▼─────────────────────────┐
│              src/agents/                    │
│  ┌─────────────┐  ┌──────────────────────┐  │
│  │ classifier/ │  │    assistant/        │  │
│  │ (intenção)  │  │  (gestão modelos)    │  │
│  └─────────────┘  └──────────────────────┘  │
│  ┌─────────────┐  ┌──────────────────────┐  │
│  │  contact/   │  │      email/          │  │
│  │   (stub)    │  │      (stub)          │  │
│  └─────────────┘  └──────────────────────┘  │
└───────────────────┬─────────────────────────┘
                    │
┌───────────────────▼─────────────────────────┐
│              src/infra/                     │
│  ┌─────────────────────────────────────┐    │
│  │    AssistantOllamaClient            │    │
│  │    (HTTP wrapper de ollama-oxide)   │    │
│  └─────────────────────────────────────┘    │
│  ┌──────────────┐  ┌───────────────────┐    │
│  │  contacts/   │  │     email/        │    │
│  │   (stub)     │  │     (stub)        │    │
│  └──────────────┘  └───────────────────┘    │
└───────────────────┬─────────────────────────┘
                    │
┌───────────────────▼─────────────────────────┐
│              src/config.rs                  │
│      Configuração lazy static (TOML)        │
└───────────────────┬─────────────────────────┘
                    │
┌───────────────────▼─────────────────────────┐
│         ollama-oxide (crate externo)        │
│    OllamaClient, ChatRequest, ChatMessage   │
└─────────────────────────────────────────────┘
```

**Regra de dependência**: cada camada só depende das camadas abaixo, nunca acima.

---

## Workspace Rust

```
ollama-ai-agents-playground/   # crate principal (bin + lib)
├── src/                       # código da aplicação
├── Cargo.toml                 # workspace root + package
└── workflowrs/                # crate de workflow (lib)
    ├── src/lib.rs
    └── Cargo.toml
```

---

## Módulos

### `src/agents/`
Camada de agentes. Cada agente implementa o trait genérico `Agent<P, T>`.

| Módulo | Status | Responsabilidade |
|--------|--------|-----------------|
| `classifier/` | Implementado | Classificação de intenção via Ollama |
| `assistant/` | Implementado | Check e criação de modelos personalizados Ollama |
| `contact/` | Stub | Gerenciamento de contatos |
| `email/` | Stub | Envio de e-mails |

### `src/infra/`
Camada de infraestrutura. Implementações técnicas sem lógica de negócio.

| Módulo | Status | Responsabilidade |
|--------|--------|-----------------|
| `assistant_ollama_client.rs` | Implementado | Wrapper do ollama-oxide para o projeto |
| `contacts/` | Stub | Persistência de contatos |
| `email/` | Stub | Envio de e-mails via SMTP |

### `src/config.rs`
Configuração TOML carregada uma vez via `once_cell`. Acesso global com `Config::get()`.

### `workflowrs/`
Biblioteca para orquestração de workflows (pendente de implementação - ver BLOCKERS.md).

---

## Padrão de Agente

```rust
pub trait Agent<P: AgentParam, T: AgentResult> {
    fn process(&self, input: P)
        -> impl Future<Output = Result<T, AgentError>> + Send;
}
```

- **`P: AgentParam`** — parâmetro de entrada tipado (marker trait)
- **`T: AgentResult`** — resultado tipado (marker trait)
- **`AgentError`** — enum de erros: `ProcessingError`, `NetworkError`, `ParseError`

### Estrutura de um módulo de agente

```
agents/{domain}/
├── mod.rs                   # Fachada: só pub mod + pub use
├── {domain}_agent.rs        # Struct + impl Agent<P, T>
├── {param}_param.rs         # Struct + impl AgentParam
└── {result}_result.rs       # Struct + impl AgentResult
```

**Princípio**: um tipo primário por arquivo. `mod.rs` sem lógica.

---

## Configuração

Acesso via singleton lazy static (inicializado uma vez em `once_cell`):

```rust
let config = Config::get();              // &'static Config
let url = config.ollama.api.chat_url(); // URL completa
let model = &config.ollama.api.model;  // "qwen3:0.6b"
```

Seções do `config.toml`:
| Seção | Conteúdo |
|-------|----------|
| `[database]` | Caminho do banco de dados |
| `[user.settings]` | Nome do usuário e do assistente |
| `[ollama.api]` | URL base, endpoints e modelo |
| `[ollama.api.options]` | Temperatura e outros parâmetros |
| `[assistant.root]` | Nome base para modelos personalizados |

---

## Padrões de Design

### Parsing de Respostas Ollama

O padrão `FromMarkdownJson<T>` extrai JSON de respostas em markdown:

```
ResponseMessage
  └── from_markdown_text(text)
        └── OllamaIntentResponseParser::from_markdown_text(text)
              └── extrai ```json...``` e desserializa
```

### Mapeamento de Tipos

O padrão `Mapper<T, U>` converte entre tipos:
- `OllamaToClassificationMapper` mapeia `ResponseMessage` → `ClassificationResult`
- `ToClassificationResult` extension trait adiciona `.to_classification_result()` ao `ResponseMessage`

### Nomeação de Modelos

```rust
// "assistant-{name}" via Config::get().assistant.root.to_name(name)
let model = build_assistant_name(&input.assistant);
```

---

## Adicionando um Novo Agente

1. Criar `src/agents/{domain}/` com `mod.rs`
2. Criar `{domain}_agent.rs`, `{param}_param.rs`, `{result}_result.rs`
3. Registrar em `src/agents/mod.rs`
4. Adicionar infra necessária em `src/infra/` se precisar de nova chamada Ollama
5. Criar testes em `tests/{domain}_tests.rs`
6. Criar exemplo em `examples/{domain}_example.rs`

Consultar `.claude/skills/agent-design/SKILL.md` para detalhes completos.
