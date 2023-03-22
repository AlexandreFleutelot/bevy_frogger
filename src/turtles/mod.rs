use bevy::prelude::*;

mod components;

mod systems;
use self::systems::*;

pub struct TurtlePlugin;

pub const TURTLE_SPRITE_SCALE: f32 = 0.8;

impl Plugin for TurtlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_turtle_spawner)
            .add_system(spawn_turtles)
            .add_system(despawn_turtles);
    }
}
