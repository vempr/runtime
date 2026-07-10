use bevy::prelude::*;

use crate::{
  GameState, components::{player::{
    Grounded, JumpRotation, Player, Velocity
  }, world::{Spike, Tile}}, constants::JUMP_SPEED, systems::setup::{spawn_level, spawn_player}
};

pub fn jump(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut query: Query<(
    &mut Velocity,
    &mut Grounded,
    &mut JumpRotation
  ), With<Player>>
) {

  if !keyboard.just_pressed(KeyCode::Space)
  && !keyboard.just_pressed(KeyCode::ArrowUp)
  && !keyboard.just_pressed(KeyCode::KeyW) {
    return;
  }

  let (mut velocity, grounded, mut rotation) = query.single_mut().unwrap();

  if grounded.0 {
    velocity.y = JUMP_SPEED;
    rotation.remaining = std::f32::consts::PI;
  }
}

pub fn restart(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  player_query: Query<Entity, (With<Player>, Without<Tile>)>,
  tile_query: Query<Entity, (With<Tile>, Without<Player>)>,
  spike_query: Query<Entity, (With<Spike>, Without<Player>)>,
  mut next_state: ResMut<NextState<GameState>>
) {
  if !keyboard.just_pressed(KeyCode::KeyR) {
    return;
  }

  for e in &player_query {
    commands.entity(e).despawn();
  }
  for e in &tile_query {
    commands.entity(e).despawn();
  }
  for e in &spike_query {
    commands.entity(e).despawn();
  }

  spawn_level(&mut commands, &mut meshes, &mut materials);
  spawn_player(&mut commands, &mut meshes, &mut materials);

  next_state.set(GameState::Playing);
}