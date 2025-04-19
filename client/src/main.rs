mod ui;
mod model {
    pub mod position;
    pub mod player;
}
mod entity;
mod network;
mod module_bindings;
use module_bindings::*;

use bevy::prelude::*;
use entity::EntityPlugin;
use ui::character::{spawn_character_creation_ui, CharacterConfig, CharacterCreationScreen, CharacterScreenPlugin};
use ui::login::{AuthState, LoginScreenPlugin, NextScreenTag};

fn main() {
    App::new()
        .insert_resource(AuthState::default())
        .insert_resource(CharacterConfig::default())
        .add_plugins(DefaultPlugins)
        .add_plugins((LoginScreenPlugin, CharacterScreenPlugin, EntityPlugin))
        .add_systems(Update, advance_to_character_creation)
        .run();
}

fn advance_to_character_creation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<NextScreenTag>>,
    character_query: Query<Entity, With<CharacterCreationScreen>>,
    config: Res<CharacterConfig>,
) {
    if !query.is_empty() && character_query.is_empty() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_character_creation_ui(&mut commands, &asset_server, &config);
    }
}
