use bevy::prelude::*;

use crate::{
  components::player::{
    Velocity,
    Grounded
  },
  constants::{
    GRAVITY,
    GROUND_Y
  }
};

pub fn process(
  time: Res<Time>,
  mut query: Query<(
    &mut Transform,
    &mut Velocity,
    &mut Grounded
  )>
) {
  let dt: f32 = time.delta_secs();

  for (mut transform, mut velocity, mut grounded) in &mut query {
    velocity.y += GRAVITY * dt;
    // transform.translation.x += velocity.x * dt;
    transform.translation.y += velocity.y * dt;

    if transform.translation.y <= GROUND_Y {
      transform.translation.y = GROUND_Y;
      velocity.y = 0.0;
      grounded.0 = true;
    } else {
      grounded.0 = false;
    }
  }
}