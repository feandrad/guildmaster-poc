use bevy::prelude::*;

#[derive(Component)]
pub struct ConnectionDialogBox;
#[derive(Component)]
pub struct ConnectionDialogCancelButton;

#[derive(Clone, Debug)]
pub enum ConnectionState {
    Connecting,
    Error(String),
    Success,
}

/// Spawns a modal dialog box overlay showing the connection state and a Cancel button.
pub fn spawn_connection_dialog(commands: &mut Commands, state: ConnectionState) {
    let (msg, color) = match &state {
        ConnectionState::Connecting => ("Connecting...", Color::YELLOW),
        ConnectionState::Error(e) => (e.as_str(), Color::RED),
        ConnectionState::Success => ("Connected!", Color::GREEN),
    };
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(0.0),
                right: Val::Percent(0.0),
                top: Val::Percent(0.0),
                bottom: Val::Percent(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.7)),
            ..default()
        },
        ConnectionDialogBox,
    )).with_children(|parent| {
        // Main dialog box with border and vertical layout
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(380.0),
                min_height: Val::Px(140.0),
                padding: UiRect::all(Val::Px(24.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.15, 0.15, 0.22)),
            border_color: BorderColor(Color::WHITE),
            ..default()
        }).with_children(|dialog| {
            dialog.spawn(TextBundle {
                text: Text::from_section(msg, TextStyle {
                    font_size: 28.0,
                    color,
                    ..default()
                }),
                style: Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            });
            // Cancel button centered at the bottom
            dialog.spawn((
                ButtonBundle {
                    style: Style {
                        margin: UiRect::top(Val::Px(8.0)),
                        align_self: AlignSelf::Center,
                        width: Val::Px(120.0),
                        height: Val::Px(36.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                },
                ConnectionDialogCancelButton,
            )).with_children(|b| {
                b.spawn(TextBundle::from_section("Cancelar", TextStyle {
                    font_size: 22.0,
                    color: Color::WHITE,
                    ..default()
                }));
            });
        });
    });
}
