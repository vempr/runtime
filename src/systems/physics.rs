use bevy::prelude::*;

use crate::{
  components::{
    player::{
      Grounded, Player, Velocity
    }, world::Tile
  }, constants::{
    GRAVITY, TILE_SIZE
  }
};

fn aabb(transform: &Transform) -> Rect {
  return Rect::from_center_size(
    transform.translation.truncate(),
    Vec2::splat(TILE_SIZE)
  );
}

pub fn process(
  time: Res<Time>,
  mut player_query: Query<(
    &mut Transform,
    &mut Velocity,
    &mut Grounded
  ), (With<Player>, Without<Tile>)>,
  tile_query: Query<&Transform, (With<Tile>, Without<Player>)>
) {
  let dt: f32 = time.delta_secs();

  let player: (Mut<'_, Transform>, Mut<'_, Velocity>, Mut<'_, Grounded>) = player_query.single_mut().unwrap();
  let (mut transform, mut velocity, mut grounded) = player;

  velocity.y += GRAVITY * dt;

  let prev_x = transform.translation.x;
  let prev_y = transform.translation.y;
  let new_y = prev_y + velocity.y * dt;

  transform.translation.x += velocity.x * dt;
  transform.translation.y = new_y;

  grounded.0 = false;
  for tile_transform in &tile_query {
    // AABB collision
    let player_rect: Rect = aabb(&transform);
    let tile_rect: Rect = aabb(tile_transform);

    if !player_rect.intersect(tile_rect).is_empty() {
      let player_bottom: f32 = player_rect.min.y;
      let player_top: f32 = player_rect.max.y;
      let tile_bottom: f32 = tile_rect.min.y;
      let tile_top: f32 = tile_rect.max.y;

      let player_right: f32 = player_rect.max.x;
      let tile_left: f32 = tile_rect.min.x;

      // player lands on tile
      if velocity.y <= 0.0 && player_bottom <= tile_top && prev_y > tile_top {
        transform.translation.y = tile_top + TILE_SIZE / 2.0;
        velocity.y = 0.0;
        grounded.0 = true;
        break;
      }

      // player hits head
      if velocity.y > 0.0 && player_top >= tile_bottom && prev_y < tile_bottom {
        println!("Player hit head! game over");
        panic!();
      }

      // player hits tile in front
      if velocity.x > 0.0 && player_right >= tile_left && prev_x + TILE_SIZE * 0.5 < tile_left {
        println!("Player hit wall! game over");
        panic!();
      }
    }
  }
}