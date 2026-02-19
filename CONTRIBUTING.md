# Guia de Contribuição — ollama-ai-agents-playground

## Requisitos

- **Rust** edition 2024 (Rust 1.85+)
- **Ollama** instalado e rodando localmente (`http://localhost:11434`)
- **Modelo**: `ollama pull qwen3:0.6b`

## Setup

```bash
git clone <repository-url>
cd ollama-ai-agents-playground

# Configurar o config.toml conforme seu ambiente
# (copie config.toml e ajuste os caminhos)

cargo build
cargo test
cargo run
```

## Estrutura do Projeto

Consulte [ARCHITECTURE.md](ARCHITECTURE.md) para a arquitetura completa.

```
src/
├── agents/         # Agentes AI (um tipo por arquivo)
├── infra/          # Infraestrutura (Ollama client)
├── config.rs       # Configuração global
└── main.rs         # Aplicação demo
tests/              # Testes de integração
examples/           # Exemplos executáveis
workflowrs/         # Crate de workflow (library)
```

## Convenções de Código

Este projeto segue a **metodologia Design Source**. As convenções estão documentadas em:

- [`.claude/skills/conventions/SKILL.md`](.claude/skills/conventions/SKILL.md) — Padrões de código Rust
- [`.claude/skills/architecture/SKILL.md`](.claude/skills/architecture/SKILL.md) — Arquitetura em camadas
- [`.claude/skills/agent-design/SKILL.md`](.claude/skills/agent-design/SKILL.md) — Design de agentes

### Regras principais

1. **Um tipo por arquivo** — cada `.rs` contém um tipo primário
2. **`mod.rs` como fachada** — apenas `pub mod` + `pub use`, sem lógica
3. **Nomenclatura** — PascalCase para tipos, snake_case para funções
4. **Erros** — variantes com sufixo `{Type}Error`
5. **Imports** — agrupados: std, externos, internos

## Adicionando um Novo Agente

Consulte [`.claude/skills/agent-design/SKILL.md`](.claude/skills/agent-design/SKILL.md) e [`.claude/skills/implementation/SKILL.md`](.claude/skills/implementation/SKILL.md).

Resumo:
1. Criar `src/agents/{domain}/` com `mod.rs`
2. Criar `{domain}_agent.rs`, `{param}_param.rs`, `{result}_result.rs`
3. Registrar em `src/agents/mod.rs`
4. Escrever testes unitários inline
5. Criar `tests/{domain}_tests.rs`
6. Criar `examples/{domain}_example.rs`

## Testes

```bash
cargo test                     # Todos os testes
cargo test classifier          # Filtrar pelo nome
cargo test -- --nocapture      # Ver output dos testes
```

Consulte [`.claude/skills/testing/SKILL.md`](.claude/skills/testing/SKILL.md) para padrões de teste.

## Build

```bash
cargo check    # Verificar sem compilar
cargo build    # Compilar
cargo clippy   # Linting
```

## Documentação

```bash
cargo doc --open   # Gerar e abrir documentação
```

Consulte [`.claude/skills/documentation/SKILL.md`](.claude/skills/documentation/SKILL.md) para padrões de documentação.

## Decisões Arquiteturais

Antes de fazer mudanças arquiteturais significativas, verifique [DECISIONS.md](DECISIONS.md) e documente sua decisão lá após implementá-la.

Para decisões pendentes ou bloqueadas, consulte e atualize [BLOCKERS.md](BLOCKERS.md).

## Pull Requests

1. Fork e crie uma branch: `feature/nome-da-feature`
2. Adicione testes para novas funcionalidades
3. Garanta que `cargo test` passa completamente
4. Garanta que `cargo check` não tem warnings desnecessários
5. Atualize `CHANGELOG.md` com suas mudanças
6. Submeta o PR

## Skills de AI (Claude Code)

Este projeto usa skills do Claude Code para desenvolvimento assistido por AI. As skills estão em `.claude/skills/` e são automaticamente usadas pelo Claude Code ao trabalhar neste projeto.
