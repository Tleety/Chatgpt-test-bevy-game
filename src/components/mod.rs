use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}