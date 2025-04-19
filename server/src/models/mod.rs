use spacetimedb::SpacetimeType;

pub mod player;
pub mod chat;
pub mod position;

#[derive(SpacetimeType, Clone, Debug)]
pub struct Color {
    pub color: String,
}