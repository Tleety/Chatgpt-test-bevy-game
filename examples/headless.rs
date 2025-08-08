// Headless example to demonstrate the game systems work
use bevy::prelude::*;
use bevy_game::{components::*, systems::movement_system};

fn main() {
    println!("Running headless Bevy 2D game simulation...");
    
    App::new()
        .add_plugins(MinimalPlugins) // No window/rendering plugins
        .add_systems(Startup, setup_headless)
        .add_systems(Update, (movement_system, log_player_position, exit_after_time))
        .run();
}

fn setup_headless(mut commands: Commands) {
    println!("Setting up headless game...");
    
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

use std::sync::atomic::{AtomicU32, Ordering};

static LOG_COUNTER: AtomicU32 = AtomicU32::new(0);

fn log_player_position(
    query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    // Log every 60 frames (roughly once per second at 60fps)
    if LOG_COUNTER.fetch_add(1, Ordering::Relaxed) % 60 == 0 {
        for transform in query.iter() {
            println!("Time: {:.1}s - Player position: ({:.2}, {:.2})", 
                time.elapsed_seconds(),
                transform.translation.x, 
                transform.translation.y
            );
            break;
        }
    }
}

fn exit_after_time(time: Res<Time>, mut exit: EventWriter<bevy::app::AppExit>) {
    if time.elapsed_seconds() > 5.0 {
        println!("Headless simulation complete after 5 seconds!");
        println!("Movement system successfully demonstrated!");
        exit.send(bevy::app::AppExit::Success);
    }
}