use bevy::prelude::*;
use crate::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                player_input_system,
                movement_system,
            ));
    }
}