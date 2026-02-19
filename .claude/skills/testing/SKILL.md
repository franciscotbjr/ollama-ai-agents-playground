# Testing Skill — ollama-ai-agents-playground

Estratégia e padrões de teste para este projeto.

---

## Organização dos Testes

```
src/
  agents/
    classifier/
      classification_result.rs   # #[cfg(test)] mod tests { ... }
      intent_param.rs            # #[cfg(test)] mod tests { ... }
tests/
  classifier_tests.rs            # Testes de integração
  assistant_tests.rs             # Testes de integração
examples/
  classify_intent.rs             # Exemplos executáveis (no_run na doc)
```

**Regras:**
- Testes **unitários**: inline com `#[cfg(test)]` no arquivo do tipo
- Testes de **integração**: em `tests/` separado
- Exemplos: em `examples/` (binários executáveis)

---

## Testes Unitários

### Estrutura

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::Intent;

    #[test]
    fn test_{type}_{action}() {
        // arrange
        // act
        // assert
    }

    #[tokio::test]
    async fn test_{type}_{action}_async() {
        // testes async
    }
}
```

### Nomenclatura: `test_{tipo}_{ação}_{variante}`

```rust
fn test_classification_result_new()
fn test_classification_result_serialize()
fn test_classification_result_deserialize_from_json()
fn test_intent_param_with_unicode()
fn test_mapper_error_parse_error()
fn test_check_param_new()
```

---

## O que testar em cada tipo

### Structs simples (Param/Result)
```rust
#[test]
fn test_check_param_new() {
    let param = CheckParam::new("model-name".to_string());
    assert_eq!(param.name, "model-name");
}

#[test]
fn test_check_param_with_empty_string() {
    let param = CheckParam::new("".to_string());
    assert_eq!(param.name, "");
}
```

### Structs com serde
```rust
#[test]
fn test_classification_result_serialize() {
    let result = ClassificationResult::new(Intent::SendEmail, Params::new());
    let json = result.to_json_string().unwrap();
    assert!(json.contains("send_email"));
}

#[test]
fn test_classification_result_deserialize() {
    let json = r#"{"intent":"send_email","params":{"recipient":null,"message":null}}"#;
    let result = ClassificationResult::from_json_str(json).unwrap();
    assert_eq!(result.intent, Intent::SendEmail);
}

#[test]
fn test_classification_result_roundtrip() {
    let original = ClassificationResult::new(Intent::NoAction, Params::new());
    let json = original.to_json_string().unwrap();
    let restored = ClassificationResult::from_json_str(&json).unwrap();
    assert_eq!(original.intent, restored.intent);
}
```

### Enums
```rust
#[test]
fn test_intent_all_variants_serialize() {
    let variants = [
        (Intent::SendEmail, "send_email"),
        (Intent::ScheduleMeeting, "schedule_meeting"),
        (Intent::NoAction, "no_action"),
    ];
    for (intent, expected) in variants {
        let json = serde_json::to_string(&intent).unwrap();
        assert!(json.contains(expected));
    }
}
```

### Parsers e Mappers
```rust
fn make_response(content: &str) -> ResponseMessage {
    serde_json::from_str(&format!(
        r#"{{"role":"assistant","content":"{}"}}"#,
        content.replace('"', r#"\""#).replace('\n', r#"\n"#)
    )).unwrap()
}

#[test]
fn test_mapper_valid_json_markdown() {
    let content = "```json\n{\"intent\":\"send_email\",\"params\":{}}\n```";
    let response = make_response(content);
    let result = response.to_classification_result().unwrap();
    assert_eq!(result.intent, Intent::SendEmail);
}

#[test]
fn test_mapper_invalid_content_returns_error() {
    let response = make_response("not json at all");
    assert!(response.to_classification_result().is_err());
}
```

### Type Safety
```rust
#[test]
fn test_check_param_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CheckParam>();
}
```

---

## Testes de Integração (`tests/`)

Os testes em `tests/` testam fluxos completos. Por enquanto, sem servidor Ollama real — testar a lógica dos agentes com mocks da resposta:

```rust
// tests/classifier_tests.rs
use ollama_ai_agents_playground::agents::classifier::{
    ClassificationResult, IntentClassifierAgent, IntentParam,
};
use ollama_ai_agents_playground::agents::Intent;

#[test]
fn test_classification_result_roundtrip() {
    let json = r#"{"intent":"send_email","params":{"recipient":"eva@test.com","message":"Hi"}}"#;
    let result = ClassificationResult::from_json_str(json).unwrap();
    assert_eq!(result.intent, Intent::SendEmail);
    assert_eq!(result.params.recipient(), Some("eva@test.com"));
}
```

---

## Cases de Borda (sempre testar)

- String vazia: `"".to_string()`
- String com Unicode: `"João 🌍 café"`
- String com aspas: `"He said \"hello\""`
- Campos `Option` como `None`
- JSON inválido / malformado
- JSON sem campos esperados

---

## Comandos

```bash
cargo test                          # Todos os testes
cargo test classifier               # Filtrar pelo nome
cargo test -- --nocapture           # Ver println! nos testes
cargo test -- --test-threads=1      # Serial (debug de problemas)
```

---

## Doc Tests

Usar `no_run` para exemplos na documentação (evitar falhas por dependência de servidor):

```rust
/// # Examples
///
/// ```no_run
/// use ollama_ai_agents_playground::agents::classifier::IntentClassifierAgent;
/// let agent = IntentClassifierAgent::new();
/// ```
pub struct IntentClassifierAgent {}
```
