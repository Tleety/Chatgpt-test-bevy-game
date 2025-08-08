use bevy::prelude::*;
use bevy_game::{components::*, plugins::GamePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D Game Example".into(),
                resolution: (800.0, 600.0).into(),
                // Prevent this from creating a new window on the web.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the player sprite (a simple colored square)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.7, 0.9),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Player,
        Velocity::default(),
    ));

    // Spawn some background elements for context
    for i in 0..5 {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.8, 0.2, 0.3),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                (i as f32 - 2.0) * 150.0,
                -200.0,
                -1.0,
            )),
            ..default()
        });
    }

    // Add some UI text instructions
    commands.spawn(
        TextBundle::from_section(
            "Use WASD or Arrow Keys to move the blue square!\nPress ESC to exit.",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}
