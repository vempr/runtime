use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy::image::ImagePlugin;

mod constants;
mod components;
mod systems;

use systems::{
  setup::setup,
  input::jump,
  physics::process
};

fn main() {
	App::new()
    .add_plugins(DefaultPlugins
      .set(ImagePlugin::default_nearest())
      .set(WindowPlugin {
        primary_window: Some(Window {
          title: "Runtime".into(),
          resolution: (1280, 720).into(),
          resizable: true,
          ..default()
        }),
        ..default()
      })
    )
    .add_systems(Startup, setup)
    .add_systems(Update, (process, jump))
    .run();
}
