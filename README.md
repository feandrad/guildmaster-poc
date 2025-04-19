# Guildmaster

Guildmaster is a multiplayer RPG with both a Rust server (SpaceTimeDB backend) and a Rust client (Bevy-based). The project supports real-time multiplayer maps, player movement, chat, and geometric area logic.

## Features
- **Map System**: Multiple maps, each with areas (rectangle, circle, triangle) and transition zones.
- **Player System**: Persistent player state (position, color, shape, etc.) using SpaceTimeDB tables.
- **Entities**: General entity support for extensible gameplay.
- **Area Logic**: Geometric queries for points in areas (collision, teleport, etc.).
- **Transition Zones**: Move between maps via defined zones.
- **Client & Server**: Rust Bevy client and SpaceTimeDB-powered server.

## Project Structure

```
server/
  src/
    lib.rs            # Main library file, reducers, module imports
    math/shape.rs     # Geometric primitives and shape logic
    maps/mod.rs       # Map, Area, and TransitionZone definitions
    maps/tavern_outside.rs # Example map definition
    models/           # Player, chat, and position models
    entities/         # Entity system
    events/           # Game event enums and logic
    systems/          # Game systems (movement, chat, map transitions, etc.)
client/
  src/
    main.rs           # Bevy game client entrypoint
    model/            # Player, position, and map models
    ui/               # Character creation, login, and game UI
    entity.rs         # Entity logic for client-side
    ...
```

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [SpaceTimeDB](https://spacetimedb.com/) (for running the database)
- [Bevy](https://bevyengine.org/) (used on the client, installed via Cargo)

### Build & Run

#### 1. Clone the repository
```sh
git clone <this-repo>
cd poc-guildmaster
```

#### 2. Start spacetimeDB
```sh
cd server
spacetime start
```

#### 3. Running the server
- In another terminal, publish the server logic to SpacetimeDB:
  ```sh
  spacetime publish --server local guildmaster
  ```

#### 4. Build the client
```sh
cd ../client
cargo build
```

#### 5. Running the client
- From the `client` directory:
  ```sh
  cargo run
  ```

## Code Highlights
- Shapes are defined in `math/shape.rs` and support geometric queries.
- Maps and areas are extensible and support multiple shapes per area.
- Transition zones let you move between maps.
- All major types are SpaceTimeDB tables for persistence and query support.
- Client and server are decoupled for flexibility.

## Contributing
Pull requests and issues are welcome! For major changes, please open an issue first to discuss what you would like to change.

## License
MIT License. See LICENSE file.
