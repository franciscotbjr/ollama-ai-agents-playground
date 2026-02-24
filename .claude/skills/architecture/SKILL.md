# Architecture Skill — ollama-ai-agents-playground

Guia de arquitetura para o projeto de orquestração de agentes AI em Rust.

---

## Arquitetura em Camadas

```
┌─────────────────────────────────┐
│         src/main.rs             │  Aplicação / Demo
├─────────────────────────────────┤
│    src/agents/                  │  Camada de Agentes
│      classifier/                │    Classificação de intenção
│      assistant/                 │    Gestão do assistente Ollama
│      contact/                   │    (stub) Contatos
│      email/                     │    (stub) E-mail
├─────────────────────────────────┤
│    src/infra/                   │  Camada de Infraestrutura
│      assistant_ollama_client.rs │    Cliente HTTP Ollama
│      contacts/                  │    (stub) Persistência
│      email/                     │    (stub) SMTP
├─────────────────────────────────┤
│    src/config.rs                │  Configuração (TOML / lazy static)
├─────────────────────────────────┤
│    ollama-oxide (crate externo) │  API Ollama
└─────────────────────────────────┘
```

**Dependências de camada (uma direção apenas):**
- `agents/` depende de `infra/` e `config`
- `infra/` depende de `config` e `ollama-oxide`
- `config` não depende de nada interno

---

## Padrão de Agente

Todo agente segue o padrão genérico:

```rust
pub trait Agent<P: AgentParam, T: AgentResult> {
    fn process(&self, input: P)
        -> impl Future<Output = Result<T, AgentError>> + Send;
}
```

- `P` — parâmetros de entrada (implementa `AgentParam`)
- `T` — resultado tipado (implementa `AgentResult`)
- `AgentError` — enum de erros padronizado

### Estrutura de um agente

Cada domínio de agente (ex: `classifier/`, `assistant/`) tem:

```
{domain}/
├── mod.rs                         # Fachada — só re-exports, sem lógica
├── {domain}_agent.rs             # Struct do agente + impl Agent<P, T>
├── {param_name}_param.rs         # Struct do parâmetro (impl AgentParam)
├── {result_name}_result.rs       # Struct do resultado (impl AgentResult)
└── ... (outros tipos auxiliares)
```

### Regra: um tipo por arquivo
Cada arquivo `.rs` contém exatamente um tipo primário (struct, enum ou trait).
O `mod.rs` de cada módulo é exclusivamente fachada de re-exports — sem lógica.

---

## Configuração

Acesso via singleton lazy static:

```rust
let config = Config::get();              // &'static Config
let url = config.ollama.api.chat_url();  // URL completa do endpoint
let model = &config.ollama.api.model;   // Nome do modelo
```

Seções do TOML:
- `[database]` — caminho do banco de dados
- `[user.settings]` — nome do usuário e do assistente
- `[ollama.api]` — URL base, endpoints e modelo
- `[ollama.api.options]` — temperatura e outros parâmetros do modelo
- `[assistant.root]` — nome base do assistente personalizado

---

## Cliente Ollama (Infra)

`AssistantOllamaClient` encapsula o `ollama-oxide`:

```rust
AssistantOllamaClient::new()
    .send_classifier_message(messages, model)   // -> Result<ChatResponse>
    .check_model_exists(name)                   // -> Result<CheckResult>
    .create_assistant(system_prompt, name)      // -> Result<OllamaCreateResponse>
```

Não implementar retry, deserialização ou HTTP manualmente — isso é responsabilidade do `ollama-oxide`.

---

## Workspace Rust

```
ollama-ai-agents-playground/   # crate principal (biblioteca + binário)
workflowrs/                    # crate de workflow (biblioteca)
```

- O crate principal exporta tipos via `src/lib.rs`
- `workflowrs` é uma biblioteca independente para orquestração de workflows
- Dependência: `workflowrs = { path = "workflowrs" }` no `Cargo.toml` principal

---

## Adicionando um novo agente

1. Criar diretório `src/agents/{domain}/`
2. Criar `{domain}_agent.rs` com a struct e `impl Agent<P, T>`
3. Criar `{param}_param.rs` com `impl AgentParam`
4. Criar `{result}_result.rs` com `impl AgentResult`
5. Criar `mod.rs` apenas com `pub mod` + `pub use`
6. Registrar o módulo em `src/agents/mod.rs`
7. Criar testes unitários inline em cada arquivo
8. Criar teste de integração em `tests/{domain}_tests.rs`
