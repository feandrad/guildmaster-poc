# AGENTS.md

## Diretório base
- O código SpacetimeDB está em `./server`

## Build
- Use `cd server && cargo build` para compilar
- Use `cd server && spacetime start` para rodar localmente

## Estilo de código
- Rodar `cd server && cargo fmt` para formatar
- Rodar `cd server && cargo clippy` antes de confirmar mudanças

## Testes
- Futuramente, usar `cd server && cargo test`

## Observações
- Eventos são definidos em `server/src/events.rs`
- Componentes: `server/src/components.rs`
- Sistemas determinísticos ficam em `server/src/systems.rs`
- Registrar todos os eventos, componentes e sistemas em `server/src/lib.rs`

## Observações
- A lógica do jogo roda no backend via sistemas determinísticos no SpacetimeDB.
- O cliente (ex: Godot) se conecta via WebSocket na porta `3000` por padrão (`ws://localhost:3000`).
