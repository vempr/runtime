use bevy::prelude::*;

use crate::{
  GameState, components::{
    player::{
      Grounded, JumpRotation, Player, Velocity
    }, world::Tile
  }, constants::{
    GRAVITY, ROTATION_SPEED, TILE_SIZE
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
    &mut Grounded,
    &mut JumpRotation
  ), (With<Player>, Without<Tile>)>,
  tile_query: Query<&Transform, (With<Tile>, Without<Player>)>,
  mut next_state: ResMut<NextState<GameState>>
) {
  let dt: f32 = time.delta_secs();

  let player: (Mut<'_, Transform>, Mut<'_, Velocity>, Mut<'_, Grounded>, Mut<'_, JumpRotation>) = player_query.single_mut().unwrap();
  let (mut transform, mut velocity, mut grounded, mut _rotation) = player;

  velocity.y += GRAVITY * dt;

  let prev_x = transform.translation.x;
  transform.translation.x += velocity.x * dt;

  for tile_transform in &tile_query {
    let player_rect: Rect = aabb(&transform);
    let tile_rect: Rect = aabb(tile_transform);

    if !player_rect.intersect(tile_rect).is_empty() {
      let player_right: f32 = player_rect.max.x;
      let tile_left: f32 = tile_rect.min.x;

      // player hits tile in front
      if velocity.x > 0.0 && player_right >= tile_left && prev_x + TILE_SIZE * 0.5 < tile_left {
        println!("Player hit wall! game over");
        next_state.set(GameState::Dead);
      }
    }
  }

  let prev_y = transform.translation.y;
  let new_y = prev_y + velocity.y * dt;
  transform.translation.y = new_y;

  let was_grounded = grounded.0;
  grounded.0 = false;
  
  for tile_transform in &tile_query {
    let player_rect: Rect = aabb(&transform);
    let tile_rect: Rect = aabb(tile_transform);

    if !player_rect.intersect(tile_rect).is_empty() {
      let player_bottom: f32 = player_rect.min.y;
      let player_top: f32 = player_rect.max.y;
      let tile_bottom: f32 = tile_rect.min.y;
      let tile_top: f32 = tile_rect.max.y;

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
        next_state.set(GameState::Dead);
      }
    }
  }

  if !grounded.0 {
    transform.rotate_z(-ROTATION_SPEED * dt);
  } else if !was_grounded && grounded.0 {
    let (_, _, angle) = transform.rotation.to_euler(EulerRot::XYZ);
    let snapped = (angle / std::f32::consts::PI).round() * std::f32::consts::PI;
    transform.rotation = Quat::from_rotation_z(snapped);
  }
}