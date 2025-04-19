use crate::math::shape::Vector2;

#[spacetimedb::table(name = entity, public)]
#[derive(Debug, Clone)]
pub struct MapEntity {
    // The `auto_inc` attribute indicates to SpacetimeDB that
    // this value should be determined by SpacetimeDB on insert.
    #[auto_inc]
    #[primary_key]
    pub entity_id: u32,
    pub position: Vector2,
    pub mass: u32,
}

#[spacetimedb::table(name = item, public)]
pub struct Item {
    #[primary_key]
    pub entity_id: u32,
}