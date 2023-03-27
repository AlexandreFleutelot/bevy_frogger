use bevy::prelude::*;

#[derive(Component)]
pub struct Turtle;

#[derive(Component)]
pub struct TurtleSpawner {
    pub sprite: Handle<Image>,
    pub transform: Transform,
    pub timer: Timer,
    pub velocity: Vec3,
    pub size: Vec2,
}
