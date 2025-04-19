use spacetime::*;

#[spacetime]
pub struct Player {
    #[primary_key]
    pub id: String,       // Derivado do token do Google (sub)
    pub name: String,
    pub color: String,    // ex: "red", "blue"
    pub shape: String,    // ex: "circle", "triangle"
    pub x: i32,
    pub y: i32,
    pub map_id: String,
}

#[spacetime]
pub struct ChatMessage {
    #[primary_key]
    pub id: i64,
    pub sender_id: String,
    pub text: String,
    pub map_id: String,
    pub timestamp: i64,
}
