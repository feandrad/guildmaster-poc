use crate::network::connection::connect_to_db;
use crate::ui::connection_dialog::{spawn_connection_dialog, ConnectionDialogBox, ConnectionDialogCancelButton, ConnectionState};
use crate::ui::fonts::{FONT_ITALIC, FONT_REGULAR};
use crate::ui::playing::spawn_playing_screen;

use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct CharacterConfig {
    pub name: String,
    pub color: Option<Color>,
    pub name_active: bool, 
}

impl Default for CharacterConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            color: None,
            name_active: false,
        }
    }
}

pub const PREDEFINED_COLORS: [Color; 8] = [
    Color::rgb(0.9, 0.2, 0.2),   // Vermelho
    Color::rgb(0.2, 0.6, 0.9),   // Azul
    Color::rgb(0.2, 0.8, 0.4),   // Verde
    Color::rgb(0.95, 0.8, 0.2),  // Amarelo
    Color::rgb(0.7, 0.3, 0.9),   // Roxo
    Color::rgb(0.9, 0.5, 0.2),   // Laranja
    Color::rgb(0.2, 0.9, 0.8),   // Ciano
    Color::rgb(0.8, 0.8, 0.8),   // Cinza claro
];

#[derive(Component)]
pub struct CharacterCreationScreen;
#[derive(Component)]
pub struct NameInputTag;
#[derive(Component)]
pub struct NameInputBoxTag;
#[derive(Component)]
pub struct ColorButtonTag(pub usize);
#[derive(Component)]
pub struct ConnectButtonTag;
#[derive(Component)]
pub struct NameHelpTextTag;

pub fn spawn_character_creation_ui(commands: &mut Commands, asset_server: &Res<AssetServer>, config: &CharacterConfig) {
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: BackgroundColor(Color::rgb(0.13, 0.13, 0.18)),
        ..default()
    }, CharacterCreationScreen)).with_children(|parent| {
        // Título
        parent.spawn(TextBundle {
            text: Text::from_section(
                "Criação de Personagem",
                TextStyle {
                    font: asset_server.load(FONT_REGULAR),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                margin: UiRect::bottom(Val::Px(24.0)),
                ..default()
            },
            ..default()
        });
        // Campo nome
        parent.spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        },)).with_children(|row| {
            row.spawn(TextBundle {
                text: Text::from_section(
                    "Nome:",
                    TextStyle {
                        font: asset_server.load(FONT_REGULAR),
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
            // Caixa do input com botão para interação
            row.spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(240.0),
                    height: Val::Px(36.0),
                    margin: UiRect::left(Val::Px(12.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.18, 0.18, 0.22)),
                border_color: BorderColor(if config.name_active { Color::rgb(0.15, 0.38, 0.85) } else { Color::GRAY }),
                ..default()
            }, NameInputBoxTag)).with_children(|input_box| {
                input_box.spawn((TextBundle {
                    text: Text::from_section(
                        if config.name.is_empty() { "Digite seu nome..." } else { &config.name },
                        TextStyle {
                            font: asset_server.load(FONT_ITALIC),
                            font_size: 24.0,
                            color: if config.name.is_empty() { Color::rgb(0.8,0.8,0.8) } else { Color::GOLD },
                        },
                    ),
                    ..default()
                }, NameInputTag));
            });
        });
        // Texto de ajuda
        parent.spawn((TextBundle {
            text: Text::from_section(
                "Nome deve conter apenas letras (A-Z, a-z), sem espaços, entre 4 e 32 caracteres.",
                TextStyle {
                    font: asset_server.load(FONT_REGULAR),
                    font_size: 14.0,
                    color: Color::rgb(0.75, 0.75, 0.8),
                },
            ),
            style: Style {
                margin: UiRect::bottom(Val::Px(18.0)),
                ..default()
            },
            ..default()
        }, NameHelpTextTag));
        // Seletor de cor
        parent.spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(18.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        },)).with_children(|row| {
            row.spawn(TextBundle {
                text: Text::from_section(
                    "Escolha sua cor:",
                    TextStyle {
                        font: asset_server.load(FONT_REGULAR),
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
            for (i, &color) in PREDEFINED_COLORS.iter().enumerate() {
                let selected = config.color.map_or(false, |c| c == color);
                row.spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        margin: UiRect::left(Val::Px(8.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(color),
                    border_color: BorderColor(if selected { Color::GOLD } else { Color::GRAY }),
                    ..default()
                }, ColorButtonTag(i)));
            }
        });
        // Botão conectar
        let name_valid = config.name.len() >= 4 && config.name.len() <= 32;
        let color_valid = config.color.is_some();
        let active = name_valid && color_valid;
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(180.0),
                height: Val::Px(48.0),
                margin: UiRect::top(Val::Px(28.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(if active { Color::rgb(0.15, 0.38, 0.85) } else { Color::rgb(0.35, 0.38, 0.45) }),
            ..default()
        }, ConnectButtonTag)).with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section(
                    "Conectar",
                    TextStyle {
                        font: asset_server.load(FONT_ITALIC),
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        });
    });
}

pub fn character_name_input(
    mut config: ResMut<CharacterConfig>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut box_query: Query<(&Interaction, &mut BorderColor), (With<NameInputBoxTag>, Changed<Interaction>)>,
    mut text_query: Query<&mut Text, With<NameInputTag>>,
) {
    // Foco visual no input box
    for (interaction, mut border) in &mut box_query {
        match *interaction {
            Interaction::Pressed => config.name_active = true,
            Interaction::Hovered => *border = BorderColor(Color::rgb(0.15, 0.38, 0.85)),
            Interaction::None => *border = BorderColor(if config.name_active { Color::rgb(0.15, 0.38, 0.85) } else { Color::GRAY }),
        }
    }
    // Input só se estiver "ativo"
    if config.name_active {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            config.name_active = false;
        } else if keyboard_input.just_pressed(KeyCode::Backspace) {
            config.name.pop();
        } else {
            let shift = keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight);
            for key in keyboard_input.get_just_pressed() {
                if let Some(c) = keycode_to_char_case(*key, shift) {
                    if c.is_ascii_alphabetic() && config.name.len() < 32 {
                        config.name.push(c);
                    }
                }
            }
        }
    }
    // Atualiza o texto
    for mut text in &mut text_query {
        text.sections[0].value = if config.name.is_empty() {
            "Digite seu nome...".to_string()
        } else {
            config.name.clone()
        };
        text.sections[0].style.color = if config.name.is_empty() { Color::rgb(0.8,0.8,0.8) } else { Color::GOLD };
    }
}

pub fn character_color_click(
    mut config: ResMut<CharacterConfig>,
    color_buttons: Query<(&Interaction, &ColorButtonTag), Changed<Interaction>>,
) {
    for (interaction, tag) in &color_buttons {
        let color = PREDEFINED_COLORS[tag.0];
        if *interaction == Interaction::Pressed {
            config.color = Some(color);
        }
    }
}

pub fn character_color_update_borders(
    config: Res<CharacterConfig>,
    mut color_buttons: Query<(&ColorButtonTag, &mut BorderColor)>,
) {
    for (tag, mut border) in &mut color_buttons {
        let color = PREDEFINED_COLORS[tag.0];
        let selected = config.color.map_or(false, |c| c == color);
        *border = BorderColor(if selected { Color::GOLD } else { Color::GRAY });
    }
}

pub fn character_color_update_connect_button(
    config: Res<CharacterConfig>,
    mut query: Query<&mut BackgroundColor, With<ConnectButtonTag>>,
) {
    let name_valid = config.name.len() >= 4 && config.name.len() <= 32 && config.name.chars().all(|c| c.is_ascii_alphabetic());
    let color_valid = config.color.is_some();
    let active = name_valid && color_valid;
    let color = if active {
        Color::rgb(0.15, 0.38, 0.85)
    } else {
        Color::rgb(0.35, 0.38, 0.45)
    };
    for mut bg in query.iter_mut() {
        *bg = BackgroundColor(color);
    }
}

#[derive(Resource, Clone, PartialEq, Eq, Default)]
pub enum AppScreen {
    #[default]
    Login,
    CharacterCreation,
    Connecting,
    Playing,
    Error(String),
}

pub fn connect_button_interaction(
    config: Res<CharacterConfig>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ConnectButtonTag>)>,
    mut commands: Commands,
    dialog_query: Query<Entity, With<ConnectionDialogBox>>,
    mut app_screen: ResMut<AppScreen>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Only spawn if not already present
            if dialog_query.iter().next().is_none() {
                spawn_connection_dialog(&mut commands, ConnectionState::Connecting);
            }
            // Call the connection logic so the reducer is triggered on the server
            let _db_connection = connect_to_db();
            *app_screen = AppScreen::Playing;
        }
    }
}

pub fn transition_to_playing_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_screen: Res<AppScreen>,
    character_query: Query<Entity, With<CharacterCreationScreen>>,
    playing_query: Query<Entity, With<crate::ui::playing::PlayingScreen>>,
) {
    if *app_screen == AppScreen::Playing && playing_query.is_empty() {
        // Remove character creation UI
        for entity in character_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        // Spawn playing screen
        spawn_playing_screen(&mut commands);
    }
}

pub fn connection_dialog_cancel_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<ConnectionDialogCancelButton>)>,
    dialog_query: Query<Entity, With<ConnectionDialogBox>>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            for dialog_entity in dialog_query.iter() {
                commands.entity(dialog_entity).despawn_recursive();
            }
        }
    }
}

fn keycode_to_char_case(key: KeyCode, shift: bool) -> Option<char> {
    let base = match key {
        KeyCode::KeyA => 'a', KeyCode::KeyB => 'b', KeyCode::KeyC => 'c', KeyCode::KeyD => 'd',
        KeyCode::KeyE => 'e', KeyCode::KeyF => 'f', KeyCode::KeyG => 'g', KeyCode::KeyH => 'h',
        KeyCode::KeyI => 'i', KeyCode::KeyJ => 'j', KeyCode::KeyK => 'k', KeyCode::KeyL => 'l',
        KeyCode::KeyM => 'm', KeyCode::KeyN => 'n', KeyCode::KeyO => 'o', KeyCode::KeyP => 'p',
        KeyCode::KeyQ => 'q', KeyCode::KeyR => 'r', KeyCode::KeyS => 's', KeyCode::KeyT => 't',
        KeyCode::KeyU => 'u', KeyCode::KeyV => 'v', KeyCode::KeyW => 'w', KeyCode::KeyX => 'x',
        KeyCode::KeyY => 'y', KeyCode::KeyZ => 'z',
        _ => return None,
    };
    if shift {
        Some(base.to_ascii_uppercase())
    } else {
        Some(base)
    }
}

// --- Plugin para a tela de criação de personagem ---
pub struct CharacterScreenPlugin;

impl Plugin for CharacterScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AppScreen>()
            .add_systems(Update, connect_button_interaction)
            .add_systems(Update, connection_dialog_cancel_system)
            .add_systems(Update, character_name_input)
            .add_systems(Update, character_color_click)
            .add_systems(Update, character_color_update_borders)
            .add_systems(Update, character_color_update_connect_button)
            .add_systems(Update, transition_to_playing_screen);
    }
}
