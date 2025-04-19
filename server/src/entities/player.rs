use crate::math::shape::Vector2;
use spacetimedb::Timestamp;

#[spacetimedb::table(name = player_entity, public)]
pub struct PlayerEntity {
    #[primary_key]
    pub entity_id: u32,
    #[index(btree)]
    pub player_id: u32,
    pub direction: Vector2,
    pub speed: f32,
    pub last_split_time: Timestamp,
}