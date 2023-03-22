mod components;
mod systems;

use bevy::prelude::*;

use self::systems::*;

const SPAWN_SHIFT: f32 = 100.;
const DESPAWN_SHIFT: f32 = 150.;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_car_spawner)
            .add_system(spawn_cars)
            .add_system(despawn_cars)
            .add_system(car_frog_collision);
    }
}
