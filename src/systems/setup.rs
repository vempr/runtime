use bevy::prelude::*;

use crate::constants::*;
use crate::components::{
  player::{
    Player,
    Velocity,
    Grounded,
    JumpRotation
  },
  world::{
    Tile
  }
};

pub fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>
) {
  let mut ortho = OrthographicProjection::default_2d();
  ortho.scaling_mode = bevy::camera::ScalingMode::AutoMin {
    min_width: VIRTUAL_WIDTH,
    min_height: VIRTUAL_HEIGHT
  };

  commands.spawn((
    Camera2d,
    Projection::Orthographic(ortho)
  ));

  // tiles for testing

  let tile_count: i32 = 30;
  for i in 0..tile_count {
    commands.spawn((
      Tile,
      Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
      MeshMaterial2d(materials.add(Color::WHITE)),
      Transform::from_xyz(i as f32 * TILE_SIZE, GROUND_Y, 0.0)
    ));
  }
  for i in 10..tile_count - 5 {
    commands.spawn((
      Tile,
      Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
      MeshMaterial2d(materials.add(Color::WHITE)),
      Transform::from_xyz(i as f32 * TILE_SIZE, GROUND_Y + 2.0 * TILE_SIZE, 0.0)
    ));
  }
  for i in tile_count..tile_count * 2 {
    commands.spawn((
      Tile,
      Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
      MeshMaterial2d(materials.add(Color::WHITE)),
      Transform::from_xyz(i as f32 * TILE_SIZE, GROUND_Y + TILE_SIZE, 0.0)
    ));
  }

  // tiles for testing

  commands.spawn((
    Player,
    Velocity {
      x: PLAYER_SPEED,
      y: 0.0
    },
    Grounded(true),
    JumpRotation::default(),
    Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
    MeshMaterial2d(materials.add(Color::WHITE)),
    Transform::default()
  ));
}