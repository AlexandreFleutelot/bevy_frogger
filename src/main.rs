use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod components;

pub mod events;
use events::GameOverEvent;

mod systems;
use systems::*;

mod player;
use player::PlayerPlugin;

mod cars;
use cars::CarPlugin;

const GRID_SIZE: (f32, f32) = (21.0, 14.0);
const TILE_SIZE: f32 = 50.0;
const TOP_BAR: f32 = 50.0;

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
        .add_plugin(WorldInspectorPlugin::new())
        .add_event::<GameOverEvent>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_background)
        .add_plugin(PlayerPlugin)
        .add_plugin(CarPlugin)
        .add_system(move_items)
        .add_system(game_over)
        .run();
}
