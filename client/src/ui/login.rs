use bevy::prelude::*;
use std::process::Command;
use urlencoding;

use crate::ui::fonts::{FONT_REGULAR, FONT_ITALIC};

#[derive(Component)]
pub struct LoginButton;

#[derive(Resource, Default)]
pub struct AuthState {
    pub logged_in: bool,
}

const GOOGLE_CLIENT_ID: &str = "520995792517-7290s8fuq07ph5mj7hmfh2bqk5277b73.apps.googleusercontent.com";
const GOOGLE_REDIRECT_URI: &str = "http://localhost:8080/callback";
const GOOGLE_SCOPE: &str = "openid email profile";

pub fn spawn_login_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // UI root node
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: BackgroundColor(Color::rgb(0.09, 0.09, 0.13)),
        ..default()
    })
    .with_children(|parent| {
        // Título
        parent.spawn(TextBundle {
            text: Text::from_section(
                "Guildmaster Login",
                TextStyle {
                    font: asset_server.load(FONT_REGULAR),
                    font_size: 48.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
            ..default()
        });
        // Botão
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(260.0),
                height: Val::Px(64.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.15, 0.38, 0.85)),
            border_color: BorderColor(Color::WHITE),
            ..default()
        }, LoginButton)).with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section(
                    "Login with Google",
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

// Placeholder para métodos OAuth2 reais (POC):
pub fn oauth2_exchange_code_for_token(_code: &str) {
    // Aqui ficaria a lógica real de troca de código por token
}

pub fn oauth2_start_callback_server() {
    // Aqui ficaria a lógica real de servidor local para callback
}

// Ao clicar, simula login e avança para a próxima tela
enum LoginState {
    Login,
    NextScreen,
}

#[derive(Component)]
pub struct NextScreenTag;

pub fn login_button_interaction(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<LoginButton>)>,
    mut auth_state: ResMut<AuthState>,
    q_next: Query<Entity, With<NextScreenTag>>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::rgb(0.10, 0.25, 0.65));
                // Simula login bem-sucedido
                auth_state.logged_in = true;
                // Remove a tela de login e mostra a próxima tela
                for entity in q_next.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                spawn_next_screen(&mut commands, &asset_server);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgb(0.20, 0.45, 0.95));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::rgb(0.15, 0.38, 0.85));
            }
        }
    }
}

pub fn spawn_next_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: BackgroundColor(Color::rgb(0.09, 0.09, 0.13)),
        ..default()
    }, NextScreenTag)).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "Bem-vindo ao Guildmaster!",
                TextStyle {
                    font: asset_server.load(FONT_REGULAR),
                    font_size: 40.0,
                    color: Color::GOLD,
                },
            ),
            ..default()
        });
    });
}

// --- Plugin para a tela de login ---
pub struct LoginScreenPlugin;
impl Plugin for LoginScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_login_ui);
        app.add_systems(Update, login_button_interaction);
    }
}
