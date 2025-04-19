pub struct TransitionZone {
    pub from_map: &'static str,
    pub to_map: &'static str,
    pub zone_rect: (i32, i32, i32, i32), // x, y, w, h
    pub target_spawn: (i32, i32),
}
