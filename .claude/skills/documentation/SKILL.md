# Documentation Skill — ollama-ai-agents-playground

Padrões de documentação para este projeto.

---

## Arquivos Obrigatórios (raiz do projeto)

| Arquivo | Propósito |
|---------|-----------|
| `README.md` | Visão geral, instalação, uso rápido |
| `ARCHITECTURE.md` | Arquitetura em camadas, módulos, padrões de design |
| `DECISIONS.md` | Log de decisões arquiteturais |
| `CHANGELOG.md` | Histórico de versões (formato Keep a Changelog) |
| `CONTRIBUTING.md` | Guia de desenvolvimento e contribuição |
| `BLOCKERS.md` | Decisões pendentes, módulos não implementados |
| `DEV_NOTES.md` | Notas de sessão de desenvolvimento (já existe) |

---

## ARCHITECTURE.md

Seções obrigatórias:
1. **Visão Geral** — propósito do projeto em 2-3 frases
2. **Arquitetura em Camadas** — diagrama ASCII da estrutura
3. **Módulos** — responsabilidade de cada módulo/crate
4. **Padrão de Agente** — como o padrão `Agent<P, T>` funciona
5. **Configuração** — como o sistema de config é usado
6. **Adicionando um Agente** — passo a passo

---

## DECISIONS.md

Formato de cada decisão:

```markdown
## [DD-001] Título da Decisão

**Data:** YYYY-MM-DD
**Status:** Aceito | Proposto | Revogado

### Contexto
Por que essa decisão foi necessária.

### Decisão
O que foi decidido.

### Consequências
- Positivas: ...
- Negativas: ...
```

---

## CHANGELOG.md

Seguir o formato [Keep a Changelog](https://keepachangelog.com/):

```markdown
# Changelog

## [Unreleased]

## [0.x.0] - YYYY-MM-DD
### Added
- ...
### Changed
- ...
### Fixed
- ...
```

---

## BLOCKERS.md

```markdown
## [BL-001] Título do Blocker

**Data:** YYYY-MM-DD
**Impacto:** Alto | Médio | Baixo

### Problema
Descrever o que está bloqueado.

### Opções
- Opção A: ...
- Opção B: ...

### Ação Necessária
Quem deve decidir / o que fazer para desbloquear.
```

---

## CONTRIBUTING.md

Incluir:
1. **Requisitos** — Rust edition 2024, Ollama local
2. **Setup** — clone, configuração do `config.toml`
3. **Build** — `cargo build`
4. **Testes** — `cargo test`
5. **Estrutura** — referência ao ARCHITECTURE.md
6. **Convenções** — referência às skills em `.claude/skills/`
7. **Adicionando Agentes** — referência ao skill `agent-design`

---

## Rustdoc

### Módulos (`//!`)
```rust
//! Módulo de classificação de intenção.
//!
//! Contém o [`IntentClassifierAgent`] que usa Ollama para classificar
//! a intenção do usuário em categorias (email, meeting, etc.).
```

### Tipos e Métodos (`///`)
```rust
/// Resultado da classificação de intenção do usuário.
///
/// Contém a intenção detectada e os parâmetros extraídos da mensagem.
///
/// # Examples
///
/// ```no_run
/// use ollama_ai_agents_playground::agents::classifier::ClassificationResult;
/// let json = r#"{"intent":"send_email","params":{"recipient":"eva@test.com","message":"Hi"}}"#;
/// let result = ClassificationResult::from_json_str(json).unwrap();
/// ```
pub struct ClassificationResult {
    pub intent: Intent,
    pub params: Params,
}
```

**Regras:**
- Todo tipo público deve ter `///`
- Exemplos na doc usam `no_run` (dependem de servidor Ollama)
- Erros documentados com seção `# Errors`
- Panics documentados com seção `# Panics`

---

## Exemplos Executáveis (`examples/`)

Nomenclatura: `{domain}_{variant}.rs`

```
examples/
├── classify_intent.rs         # Fluxo de classificação de intenção
├── check_assistant.rs         # Verificar existência do assistente
└── create_assistant.rs        # Criar assistente personalizado
```

Cada exemplo:
1. Inicializa o agente
2. Cria o parâmetro de entrada
3. Chama `agent.process(param).await`
4. Imprime o resultado

Registrar no `Cargo.toml`:
```toml
[[example]]
name = "classify_intent"
path = "examples/classify_intent.rs"
```
