use spacetime::*;

#[spacetime]
pub enum GameEvent {
    Move { player_id: String, dx: i32, dy: i32 },
    SpawnPlayer { player_id: String, name: String, color: String, shape: String },
    ChatMessage { player_id: String, text: String },
    ChangeMap { player_id: String, new_map_id: String, new_x: i32, new_y: i32 },
}
