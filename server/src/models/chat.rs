#[spacetimedb::table(name = chat_message)]
pub struct ChatMessage {
    pub id: i64,
    pub sender_id: String,
    pub text: String,
    pub map_id: String,
    pub timestamp: i64,
}
