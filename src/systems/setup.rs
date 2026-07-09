use bevy::prelude::*;

use crate::constants::*;
use crate::components::player::{
  Player,
  Velocity,
  Grounded
};

pub fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>
) {
  commands.spawn(Camera2d);

  commands.spawn((
    Player,
    Velocity::default(),
    Grounded(true),
    Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
    MeshMaterial2d(materials.add(Color::WHITE)),
    Transform::default()
  ));
}

