use bevy::prelude::*;

use crate::{
  constants::JUMP_SPEED,
  components::player::{
    Player,
    Velocity,
    Grounded
  }
};

pub fn jump(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut query: Query<(
    &mut Velocity,
    &mut Grounded
  ), With<Player>>
) {
  if !keyboard.just_pressed(KeyCode::Space) {
    return;
  }

  let (mut velocity, grounded) = query.single_mut().unwrap();

  if grounded.0 {
    velocity.y = JUMP_SPEED;
  }
}