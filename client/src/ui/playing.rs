use bevy::prelude::*;

#[derive(Component)]
pub struct PlayingScreen;

pub fn spawn_playing_screen(commands: &mut Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.13, 0.13, 0.18)),
            ..default()
        },
        PlayingScreen,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Gameplay goes here!",
            TextStyle {
                font_size: 36.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}
