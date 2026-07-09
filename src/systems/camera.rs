use bevy::prelude::*;

use crate::components::player::Player;

pub fn follow_player(
  mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
  player: Query<&Transform, With<Player>>
) {
  let player: &Transform = player.single().unwrap();
  let mut cam: Mut<'_, Transform> = camera.single_mut().unwrap();

  cam.translation.x = player.translation.x;
}