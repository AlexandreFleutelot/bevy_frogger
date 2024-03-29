use bevy::prelude::*;

#[derive(Component)]
pub struct Car;

#[derive(Component)]
pub struct CarSpawner {
    pub sprite: Handle<Image>,
    pub transform: Transform,
    pub timer: Timer,
    pub velocity: Vec3,
    pub size: Vec2,
}
