# Agent Design Skill — ollama-ai-agents-playground

Guia para projetar e implementar novos agentes AI neste projeto.

---

## Anatomia de um Agente

Um agente é composto por 3 elementos fundamentais:

```
{Domain}Agent     →  implementa Agent<{Domain}Param, {Domain}Result>
{Domain}Param     →  implementa AgentParam (entrada tipada)
{Domain}Result    →  implementa AgentResult (saída tipada)
```

### Exemplo: EmailAgent

```rust
// email_agent.rs
pub struct EmailAgent {}

impl Agent<EmailParam, EmailResult> for EmailAgent {
    fn process(&self, input: EmailParam)
        -> impl Future<Output = Result<EmailResult, AgentError>> + Send
    {
        async move {
            // lógica do agente
            Ok(EmailResult::new(true))
        }
    }
}

// email_param.rs
pub struct EmailParam {
    pub recipient: String,
    pub message: String,
}
impl AgentParam for EmailParam {}

// email_result.rs
pub struct EmailResult {
    pub success: bool,
}
impl AgentResult for EmailResult {}
```

---

## Prompt Engineering com AgentPromptBuilder

Use `AgentPromptBuilder` para construir prompts estruturados:

```rust
fn build_system_prompt() -> String {
    AgentPrompt::builder()
        .add_instruction(YOU_ARE)
        .add_instruction(LINE)
        .add_instruction(OUTPUT_FORMAT)
        .add_instruction(LINE)
        .add_instruction(EXAMPLE)
        .build()
        .content()
        .to_string()
}

const YOU_ARE: &str = "You are a helpful assistant that...";
const OUTPUT_FORMAT: &str = "Output-Format: {...}";
const EXAMPLE: &str = "Example: Input: ... Output: ...";
const LINE: &str = "\n";
```

**Regras de prompt:**
- Separar system prompt (instruções) do user prompt (entrada do usuário)
- System prompt: identidade, formato de saída, exemplos
- User prompt: dados concretos da requisição atual
- Usar constantes nomeadas para partes reutilizáveis do prompt

---

## Interação com Ollama

Os agentes interagem com Ollama através do `AssistantOllamaClient`:

```rust
// Enviar mensagens de chat
let response = AssistantOllamaClient::new()
    .send_classifier_message(
        vec![
            ChatMessage::system(system_prompt),
            ChatMessage::user(user_prompt),
        ],
        model_name,
    )
    .await?;

// Verificar existência de modelo
let check = AssistantOllamaClient::new()
    .check_model_exists(model_name)
    .await?;

// Criar modelo personalizado
let result = AssistantOllamaClient::new()
    .create_assistant(system_prompt, model_name)
    .await?;
```

---

## Parsing de Respostas

Para respostas em formato JSON dentro de markdown, use `FromMarkdownJson`:

```rust
impl FromMarkdownJson<ClassificationResult> for OllamaIntentResponseParser {
    fn from_markdown_text(text: &str) -> Result<ClassificationResult, Box<dyn Error>> {
        let json = extract_json_from_markdown(text)?;
        Ok(serde_json::from_str(&json)?)
    }
}
```

`ResponseMessage` também implementa `FromMarkdownJson<ClassificationResult>`, delegando ao parser.

---

## Mapeamento de Respostas

Use o padrão `Mapper<T, U>` para converter entre tipos:

```rust
pub trait Mapper<T, U> {
    type Error;
    fn map(source: T) -> Result<U, Self::Error>;
}

pub struct OllamaToClassificationMapper;

impl Mapper<&ResponseMessage, ClassificationResult> for OllamaToClassificationMapper {
    type Error = MapperError;
    fn map(source: &ResponseMessage) -> Result<ClassificationResult, MapperError> { ... }
}
```

Extension traits adicionam conveniência:

```rust
pub trait ToClassificationResult {
    fn to_classification_result(&self) -> Result<ClassificationResult, MapperError>;
}

impl ToClassificationResult for ResponseMessage {
    fn to_classification_result(&self) -> Result<ClassificationResult, MapperError> {
        OllamaToClassificationMapper::map(self)
    }
}
```

---

## Tratamento de Erros nos Agentes

Sempre converter erros externos para `AgentError`:

```rust
impl Agent<CheckParam, CheckResult> for CheckAssistantAgent {
    fn process(&self, input: CheckParam) -> impl Future<...> + Send {
        async move {
            AssistantOllamaClient::new()
                .check_model_exists(&input.name)
                .await
                .map(|r| CheckResult::new(r.exists))
                .map_err(|e| AgentError::NetworkError(e.to_string()))
        }
    }
}
```

Mapeamento de erros:
- Falhas HTTP / rede → `AgentError::NetworkError`
- Falhas de parsing / deserialização → `AgentError::ParseError`
- Falhas de lógica de negócio → `AgentError::ProcessingError`

---

## Nomear Modelos Ollama

Use `build_assistant_name` para construir o nome do modelo personalizado:

```rust
// Usa Config::get().assistant.root.to_name(name)
// Resultado: "assistant-{name}"
let model = build_assistant_name(&input.assistant);
```

---

## Checklist para Novo Agente

- [ ] Criar `src/agents/{domain}/` com `mod.rs`
- [ ] Criar `{domain}_agent.rs` — struct + `impl Agent<P, T>`
- [ ] Criar `{param}_param.rs` — `impl AgentParam`
- [ ] Criar `{result}_result.rs` — `impl AgentResult`
- [ ] Atualizar `src/agents/{domain}/mod.rs` com re-exports
- [ ] Registrar módulo em `src/agents/mod.rs`
- [ ] Adicionar infraestrutura necessária em `src/infra/`
- [ ] Escrever testes unitários em cada arquivo
- [ ] Criar `tests/{domain}_tests.rs` com testes de integração
- [ ] Adicionar exemplo em `examples/{domain}_example.rs`
