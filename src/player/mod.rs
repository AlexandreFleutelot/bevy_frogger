pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_frog)
            .add_systems((frog_controller, confine_player).chain());
    }
}
