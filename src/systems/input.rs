use bevy::prelude::*;

use crate::constants::TILE_SIZE;
use crate::components::player::Player;

pub fn jump(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut query: Query<&mut Transform, With<Player>>
) {
  if keyboard.just_pressed(KeyCode::Space) {
    println!("jump!");
    if let Ok(mut trans) = query.single_mut() {
      trans.translation.y += TILE_SIZE;
    }
  }
}