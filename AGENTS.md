# AGENTS.md

## Build
- Use `cargo build` para compilar.
- Use `spacetime dev` para rodar localmente.

## Estilo de código
- Formate com `cargo fmt`
- Rodar `cargo clippy` antes de confirmar mudanças

## Testes
- Testes podem ser adicionados com `cargo test` futuramente.

## Observações
- Código determinístico: sistemas SpacetimeDB devem evitar aleatoriedade.
- Eventos precisam ser registrados em `lib.rs`
