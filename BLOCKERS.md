# Blockers — ollama-ai-agents-playground

Decisões pendentes e módulos não implementados.

---

## [BL-001] EmailAgent — implementação pendente

**Data:** 2025-08-30
**Impacto:** Alto

### Problema
`src/agents/email/` e `src/infra/email/` existem como stubs. O `EmailAgent` não tem implementação real — sem envio de e-mail, sem validação, sem SMTP.

### Opções
- **Opção A**: Integrar com SMTP via crate `lettre`
- **Opção B**: Integrar com API de e-mail externa (SendGrid, Mailgun)
- **Opção C**: Simular envio para fins de demo (log apenas)

### Ação Necessária
Decidir o mecanismo de envio de e-mail antes de implementar.

---

## [BL-002] ContactAgent — implementação pendente

**Data:** 2025-08-30
**Impacto:** Médio

### Problema
`src/agents/contact/` e `src/infra/contacts/` existem como stubs. Não há forma de buscar contatos, resolver nomes para e-mails, ou persistir informações de contato.

### Opções
- **Opção A**: Arquivo JSON local de contatos
- **Opção B**: SQLite via `rusqlite` (o `config.toml` já tem `database.path`)
- **Opção C**: Integrar com serviço externo (Google Contacts, etc.)

### Ação Necessária
Decidir o mecanismo de persistência de contatos.

---

## [BL-003] PersonalAssistant — implementação pendente

**Data:** 2025-08-30
**Impacto:** Alto

### Problema
`src/assistant/personal_assistant.rs` está vazio. A intenção é criar um assistente de alto nível que orquestre os agentes (classifier → email/contact/etc.), mas a API não está definida.

### Opções
- **Opção A**: `PersonalAssistant` como struct com método `handle(input: &str) -> Result<AssistantResponse>`
- **Opção B**: Usar `workflowrs` para orquestrar o fluxo
- **Opção C**: Implementar diretamente em `main.rs` (sem abstração de alto nível)

### Ação Necessária
Definir a API do `PersonalAssistant` antes de implementar. Relacionado com BL-004.

---

## [BL-004] workflowrs — API não definida

**Data:** 2025-09-12
**Impacto:** Médio

### Problema
`workflowrs/` existe como placeholder com apenas `src/main.rs` (deveria ser `lib.rs`). Não há definição de como workflows serão modelados: steps, condições, paralelismo, etc.

### Opções
- **Opção A**: Pipeline linear: `Input → Step1 → Step2 → Output`
- **Opção B**: DAG (Directed Acyclic Graph) de steps com condições
- **Opção C**: Baseado em eventos: agentes reagem a eventos publicados

### Ação Necessária
Decidir o modelo de orquestração de workflow antes de implementar a API pública da crate.

---

## [BL-005] Testes de integração com Ollama real

**Data:** 2026-02-18
**Impacto:** Médio

### Problema
Os testes de integração em `tests/` não podem chamar o Ollama real (requer servidor rodando). Precisamos de uma estratégia de mock ou de testes de integração marcados como `#[ignore]`.

### Opções
- **Opção A**: Usar `#[ignore]` para testes que precisam do Ollama, rodar manualmente
- **Opção B**: Criar trait `OllamaClientTrait` para mockabilidade do `AssistantOllamaClient`
- **Opção C**: Usar `mockito` para mockar as chamadas HTTP do ollama-oxide

### Ação Necessária
Escolher estratégia de mock antes de criar testes de integração end-to-end.
