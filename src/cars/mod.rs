mod components;
mod systems;

use bevy::prelude::*;

use self::systems::*;

const CAR_SPRITE_SCALE: f32 = 0.7;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_car_spawner)
            .add_system(spawn_cars)
            .add_system(despawn_cars)
            .add_system(car_frog_collision);
    }
}
