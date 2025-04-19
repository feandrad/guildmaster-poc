use crate::maps::Coords;

#[spacetimedb::table(name = player)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub shape: String,
    pub coords: Coords,
}
