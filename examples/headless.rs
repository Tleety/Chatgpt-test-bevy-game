// Headless example to demonstrate the game systems work
use bevy::prelude::*;
use bevy_game::{components::*, systems::movement_system};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins) // No window/rendering plugins
        .add_systems(Startup, setup_headless)
        .add_systems(Update, (movement_system, exit_after_time))
        .run();
}

fn setup_headless(mut commands: Commands) {
    // Create the player entity without any rendering components
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Player,
        Velocity { x: 50.0, y: 25.0 }, // Start with some velocity
    ));
    
    // Create some other entities to show the system works
    for i in 0..3 {
        commands.spawn((
            Transform::from_translation(Vec3::new(i as f32 * 10.0, 0.0, 0.0)),
            Velocity { x: -20.0, y: 10.0 },
        ));
    }
}


fn exit_after_time(time: Res<Time>, mut exit: EventWriter<bevy::app::AppExit>) {
    if time.elapsed_seconds() > 5.0 {
        exit.send(bevy::app::AppExit::Success);
    }
}