use bevy::prelude::*;

#[derive(Component)]
pub struct Trunk;

#[derive(Component)]
pub struct TrunkSpawner {
    pub sprite: Handle<Image>,
    pub transform: Transform,
    pub timer: Timer,
    pub velocity: Vec3,
    pub size: Vec2,
}
