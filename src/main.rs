use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy::image::ImagePlugin;

mod constants;
mod components;
mod systems;

use systems::{
  setup::setup,
  input::jump,
  input::restart,
  physics::process,
  camera::follow_player
};

#[derive(States, Default, Hash, Clone, Debug, Eq, PartialEq)]
enum GameState {
  #[default]
  Playing,
  Dead,
  MainMenu,
  Pause,
  Loading
}

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
    .init_state::<GameState>()
    .add_systems(Startup, setup)
    .add_systems(
      Update,
      (
        process,
        jump,
        follow_player
      )
      .chain()
      .run_if(in_state(GameState::Playing))
    )
    .add_systems(
      Update,
      restart
    )
    .run();
}
