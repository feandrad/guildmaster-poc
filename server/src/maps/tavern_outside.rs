

// Example static values (can be used to initialize tables)
pub const STARTER_MAP_ID: &str = "core:tavern:outside";
pub const TAVERN_MAP_ID: &str = "core:tavern:inside";
pub const STARTER_MAP_WIDTH: i32 = 1000;
pub const STARTER_MAP_HEIGHT: i32 = 1000;
pub const ARRIVAL_X: i32 = 250;
pub const ARRIVAL_Y: i32 = STARTER_MAP_HEIGHT / 2;

// Example for initializing a teleport zone
pub const TELEPORT_ZONE_X: i32 = STARTER_MAP_WIDTH / 2 - 20;
pub const TELEPORT_ZONE_Y: i32 = STARTER_MAP_HEIGHT / 2 - 300 - 20;
pub const TELEPORT_ZONE_W: i32 = 40;
pub const TELEPORT_ZONE_H: i32 = 40;
pub const TELEPORT_TARGET_X: i32 = 100;
pub const TELEPORT_TARGET_Y: i32 = 100;

#[spacetimedb::table(name = starter_area)]
pub struct StarterArea {
    pub id: String, // usually map_id
    pub width: i32,
    pub height: i32,
    pub arrival_x: i32,
    pub arrival_y: i32,
}
