use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Velocity {
  pub x: f32,
  pub y: f32,
}

#[derive(Component, Default)]
pub struct Grounded(pub bool);

#[derive(Component, Default)]
pub struct JumpRotation {
  pub remaining: f32
}