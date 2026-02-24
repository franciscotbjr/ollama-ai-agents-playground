# Implementation Skill — ollama-ai-agents-playground

Workflow e checklists para implementar funcionalidades neste projeto.

---

## Workflow de Implementação de Agente

### Passo 1: Tipos (um tipo por arquivo)

```rust
// src/agents/{domain}/{param}_param.rs
#[derive(Debug, Clone)]
pub struct {Domain}Param {
    pub field: String,
}
impl AgentParam for {Domain}Param {}

// src/agents/{domain}/{result}_result.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Domain}Result {
    pub success: bool,
}
impl AgentResult for {Domain}Result {}
```

### Passo 2: Fachada do Módulo

```rust
// src/agents/{domain}/mod.rs — APENAS re-exports, sem lógica
pub mod {domain}_agent;
pub mod {param}_param;
pub mod {result}_result;

pub use {domain}_agent::{Domain}Agent;
pub use {param}_param::{Domain}Param;
pub use {result}_result::{Domain}Result;
```

### Passo 3: Implementar o Agente

```rust
// src/agents/{domain}/{domain}_agent.rs
pub struct {Domain}Agent {}

impl {Domain}Agent {
    pub fn new() -> Self { Self {} }
}

impl Agent<{Domain}Param, {Domain}Result> for {Domain}Agent {
    fn process(&self, input: {Domain}Param)
        -> impl Future<Output = Result<{Domain}Result, AgentError>> + Send
    {
        async move {
            // 1. Construir prompts (se aplicável)
            // 2. Chamar AssistantOllamaClient
            // 3. Mapear resultado para {Domain}Result
            // 4. Converter erros para AgentError
        }
    }
}
```

### Passo 4: Registrar no Módulo Pai

```rust
// src/agents/mod.rs
pub mod {domain};
// pub use {domain}::{Domain}Agent;  // se necessário exportar no nível raiz
```

### Passo 5: Infraestrutura (se necessário)

Adicionar métodos no `AssistantOllamaClient` se o agente precisar de nova chamada à API Ollama:

```rust
// src/infra/assistant_ollama_client.rs
pub async fn {new_operation}(&self, ...) -> Result<..., ...> {
    let client = OllamaClient::with_base_url(&self.base_url)?;
    // usar ollama-oxide para a chamada
}
```

### Passo 6: Testes Unitários

Adicionar em cada arquivo novo (dentro do `#[cfg(test)] mod tests`):
- Teste de construção do tipo
- Teste de serialização/deserialização (se aplicável)
- Teste de cada método público
- Teste de casos de borda (strings vazias, None, Unicode)

### Passo 7: Testes de Integração

```rust
// tests/{domain}_tests.rs
#[tokio::test]
async fn test_{domain}_agent_process() {
    // Mock ou teste real com servidor Ollama local
}
```

### Passo 8: Exemplo Executável

```rust
// examples/{domain}_example.rs
#[tokio::main]
async fn main() {
    let agent = {Domain}Agent::new();
    let param = {Domain}Param::new("...".to_string());
    let result = agent.process(param).await.unwrap();
    println!("{:?}", result);
}
```

Adicionar ao `Cargo.toml`:
```toml
[[example]]
name = "{domain}_example"
path = "examples/{domain}_example.rs"
```

---

## Workflow de Divisão de Arquivos (Refactoring)

Quando um arquivo tem múltiplos tipos:

1. Identificar todos os tipos no arquivo
2. Para cada tipo secundário:
   a. Criar novo arquivo com o nome do tipo
   b. Mover o tipo e seus `impl` blocks
   c. Ajustar imports no novo arquivo
   d. Ajustar imports no arquivo original
3. Atualizar o `mod.rs` com os novos módulos e re-exports
4. Executar `cargo check` para verificar
5. Executar `cargo test` para garantir que testes passam

---

## Checklist Pré-Commit

- [ ] `cargo check` sem erros
- [ ] `cargo test` — todos os testes passam
- [ ] `cargo build` sem warnings
- [ ] Um tipo por arquivo (verificar arquivos novos/modificados)
- [ ] `mod.rs` sem lógica — apenas re-exports
- [ ] Erros mapeados para variantes corretas de `AgentError`
- [ ] Testes inline adicionados para código novo
- [ ] Tipos públicos documentados com `///`

---

## Padrões Comuns

### Construtor simples
```rust
pub fn new(field: String) -> Self { Self { field } }
```

### With-method chain (campos opcionais)
```rust
pub fn with_recipient(mut self, r: impl Into<String>) -> Self {
    self.recipient = Some(r.into());
    self
}
```

### Converter erro externo para AgentError
```rust
.map_err(|e| AgentError::NetworkError(e.to_string()))
.map_err(|e| AgentError::ParseError(e.to_string()))
.map_err(|e| AgentError::ProcessingError(e.to_string()))
```

### Prompt com constantes
```rust
const LINE: &str = "\n";
const YOU_ARE: &str = "You are...";

fn build_prompt() -> String {
    AgentPrompt::builder()
        .add_instruction(YOU_ARE)
        .add_instruction(LINE)
        .build()
        .content()
        .to_string()
}
```
