use crate::model::player::Player;
use crate::model::position::Position;
use bevy::prelude::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
           .register_type::<Position>();
    }
}
