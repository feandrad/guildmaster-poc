mod ui;

use bevy::prelude::*;
use ui::login::{AuthState, spawn_login_ui, login_button_interaction, spawn_next_screen, NextScreenTag, LoginScreenPlugin};
use ui::character::{CharacterConfig, spawn_character_creation_ui, character_name_input, character_color_click, character_color_update_borders, connect_button_interaction, CharacterCreationScreen, CharacterScreenPlugin};

fn main() {
    App::new()
        .insert_resource(AuthState::default())
        .insert_resource(CharacterConfig::default())
        .add_plugins(DefaultPlugins)
        .add_plugins((LoginScreenPlugin, CharacterScreenPlugin))
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
        // Remove a tela de boas-vindas
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        // Mostra a tela de criação de personagem
        spawn_character_creation_ui(&mut commands, &asset_server, &config);
    }
}
// Troque .add_plugin por .add_plugins para múltiplos plugins, conforme a API do Bevy 0.13
