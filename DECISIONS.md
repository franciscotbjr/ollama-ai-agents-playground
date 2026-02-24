# Decisões Arquiteturais — ollama-ai-agents-playground

Registro de decisões importantes tomadas no projeto.

---

## [DD-001] Uso do ollama-oxide como cliente Ollama

**Data:** 2025-09-12
**Status:** Aceito

### Contexto
O projeto precisava de um cliente HTTP para a API Ollama. Inicialmente usava implementação própria com `reqwest`.

### Decisão
Migrar para o crate `ollama-oxide` (v0.1.2) como cliente Ollama, encapsulado pelo `AssistantOllamaClient`.

### Consequências
- Positivas: API bem tipada, retry embutido, menos código a manter
- Positivas: `ChatMessage`, `ChatRequest`, `ChatResponse` já definidos
- Negativas: Dependência de crate externo (acoplamento)
- Negativas: Feature `"model"` necessária para `show_model` e `create`

---

## [DD-002] Trait genérico Agent<P, T>

**Data:** 2025-08-30
**Status:** Aceito

### Contexto
Precisávamos de um padrão para criar agentes AI extensíveis com tipos seguros para entrada e saída.

### Decisão
Definir `Agent<P: AgentParam, T: AgentResult>` com trait bounds e resultado async:

```rust
pub trait Agent<P: AgentParam, T: AgentResult> {
    fn process(&self, input: P) -> impl Future<Output = Result<T, AgentError>> + Send;
}
```

### Consequências
- Positivas: Compile-time type safety, cada agente tem tipos explícitos
- Positivas: Extensível — novos agentes apenas implementam o trait
- Negativas: Não é object-safe (não pode ser usado em `dyn Agent<_, _>` diretamente)

---

## [DD-003] Configuração TOML com lazy static (once_cell)

**Data:** 2025-08-30
**Status:** Aceito

### Contexto
O projeto precisa de configuração acessível globalmente (URL Ollama, modelo, configurações do usuário).

### Decisão
Usar `once_cell::sync::OnceCell` com `Config::get()` retornando `&'static Config`. Arquivo `config.toml` na raiz do projeto.

### Consequências
- Positivas: Acesso sem boilerplate em qualquer módulo
- Positivas: Inicialização única, thread-safe
- Negativas: Config não pode ser alterada em runtime
- Negativas: Testes que precisam de configs diferentes requerem arquivos TOML temporários

---

## [DD-004] Modelo qwen3:0.6b para classificação de intenção

**Data:** 2025-08-30
**Status:** Aceito

### Contexto
Testados múltiplos modelos: `deepseek-r1:1.5b`, `llama3.1`, `mistral-nemo`, `gemma3`, `qwen3:0.6b`.

### Decisão
Usar `qwen3:0.6b` como modelo padrão para classificação de intenção.

### Consequências
- Positivas: Menor consumo de recursos, respostas mais rápidas
- Positivas: Melhor consistência nas classificações entre os modelos testados
- Negativas: Modelo menor pode falhar em casos complexos ou ambíguos

---

## [DD-005] Temperature = 0.0 para classificação

**Data:** 2025-08-30
**Status:** Aceito

### Contexto
Classificação de intenção precisa ser determinística — mesma entrada deve sempre produzir mesma saída.

### Decisão
Configurar `temperature = 0` no `config.toml` para eliminar aleatoriedade.

### Consequências
- Positivas: Resultados idempotentes, comportamento previsível
- Negativas: Zero criatividade nas respostas (aceitável para classificação)

---

## [DD-006] Two-message structure (system + user)

**Data:** 2025-08-30
**Status:** Aceito

### Contexto
Experimentos mostraram que incluir instruções e input do usuário na mesma mensagem produzia resultados inconsistentes.

### Decisão
Separar o prompt em dois papéis distintos:
- `ChatMessage::system(...)` — instruções, formato de saída, exemplos
- `ChatMessage::user(...)` — input real do usuário

### Consequências
- Positivas: Classificações significativamente mais consistentes
- Positivas: Separação clara entre instrução e dado
- Negativas: Nenhuma conhecida

---

## [DD-007] Workspace com crate workflowrs

**Data:** 2025-09-12
**Status:** Aceito (implementação pendente — ver BLOCKERS.md)

### Contexto
Orquestração de múltiplos agentes requer um sistema de workflow separado da lógica de cada agente.

### Decisão
Criar `workflowrs` como crate independente no workspace para orquestração de workflows.

### Consequências
- Positivas: Separação de responsabilidades, reutilizável por outros projetos
- Positivas: Pode ser publicado no crates.io independentemente
- Negativas: Ainda não implementado (placeholder)

---

## [DD-008] Aplicar metodologia Design Source

**Data:** 2026-02-18
**Status:** Aceito

### Contexto
O projeto cresceu organicamente sem um padrão de organização explícito. Precisávamos padronizar file organization, documentação e AI-assisted workflow.

### Decisão
Aplicar a metodologia Design Source ao projeto: um tipo por arquivo, skills do Claude em `.claude/skills/`, documentação obrigatória (ARCHITECTURE, DECISIONS, CHANGELOG, BLOCKERS), testes de integração em `tests/`.

### Consequências
- Positivas: Consistência, manutenibilidade, AI-assisted development padronizado
- Positivas: Onboarding mais fácil com skills e docs completas
- Negativas: Refactoring necessário (divisão de arquivos com múltiplos tipos)
