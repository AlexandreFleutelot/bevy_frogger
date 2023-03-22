use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

#[derive(Component)]
pub struct Spawner {
    pub sprite: Handle<Image>,
    pub transform: Transform,
    pub timer: Timer,
    pub velocity: Vec3,
    pub size: Vec2,
}
