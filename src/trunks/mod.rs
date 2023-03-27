use bevy::prelude::*;

pub mod components;

mod systems;
use self::systems::*;

pub struct TrunkPlugin;

pub const LOG_SPRITE_SCALE: f32 = 0.8;

impl Plugin for TrunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_trunk_spawner)
            .add_system(spawn_trunks)
            .add_system(despawn_trunks);
    }
}
