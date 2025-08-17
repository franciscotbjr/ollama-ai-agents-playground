  1. Implementar Estruturas Core:
  // Intent e parâmetros conforme especificação
  pub enum Intent { SendEmail, ScheduleMeeting, NoAction }
  pub struct ClassificationResult { intent: Intent, params:
  serde_json::Value }
  2. Completar Trait Agent:
  pub trait Agent {
      async fn process(&self, input: &str) -> Result<String,
  AgentError>;
  }
  3. Implementar Cliente Ollama:
    - Estruturas para request/response da API
    - Métodos para envio de mensagens
    - Parsing do JSON de resposta
  4. Pipeline de Classificação:
    - Prompt engineering conforme especificação
    - Extração de JSON da resposta
    - Validação de parâmetros

  Conclusão: A estrutura básica está bem organizada mas precisa       
  de implementação funcional para atender plenamente à
  especificação.