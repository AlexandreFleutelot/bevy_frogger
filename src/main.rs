pub mod components;
mod player;
mod systems;

use bevy::prelude::*;

use player::PlayerPlugin;
use systems::spawn_camera;

const GRID_SIZE: (f32, f32) = (21.0, 16.0);
const TILE_SIZE: f32 = 50.0;

fn main() {
    let window = Window {
        resolution: (TILE_SIZE * GRID_SIZE.0, TILE_SIZE * GRID_SIZE.1).into(),
        resizable: false,
        ..default()
    };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // prevents blurry sprites
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                }),
        )
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .run();
}
