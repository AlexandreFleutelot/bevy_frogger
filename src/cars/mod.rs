mod components;
mod systems;

use bevy::prelude::*;

use self::systems::*;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(set_car_spawner)
        .add_system(spawn_cars);
    }
}