use bevy::prelude::*;

use crate::{
  components::player::{
    Grounded, JumpRotation, Player, Velocity
  }, constants::JUMP_SPEED
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