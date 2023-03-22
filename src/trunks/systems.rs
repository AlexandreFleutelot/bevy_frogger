use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::{SpriteSize, Velocity};
use crate::{DESPAWN_SHIFT, SPAWN_SHIFT, TILE_SIZE};

use super::components::*;

use super::LOG_SPRITE_SCALE;

pub fn set_trunk_spawner(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TrunkSpawner {
        sprite: asset_server.load("small_log.png"),
        transform: Transform {
            translation: Vec3::new(-SPAWN_SHIFT, 8.5 * TILE_SIZE as f32, 0.),
            rotation: Quat::from_rotation_z(0.),
            scale: Vec3::ONE * LOG_SPRITE_SCALE,
        },
        timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        velocity: Vec3::new(60., 0., 0.),
        size: Vec2::new(200., 70.) * LOG_SPRITE_SCALE,
    });

    commands.spawn(TrunkSpawner {
        sprite: asset_server.load("medium_log.png"),
        transform: Transform {
            translation: Vec3::new(-SPAWN_SHIFT, 11.5 * TILE_SIZE as f32, 0.),
            rotation: Quat::from_rotation_z(0.),
            scale: Vec3::ONE * LOG_SPRITE_SCALE,
        },
        timer: Timer::from_seconds(6.7, TimerMode::Repeating),
        velocity: Vec3::new(60., 0., 0.),
        size: Vec2::new(280., 70.) * LOG_SPRITE_SCALE,
    });

    commands.spawn(TrunkSpawner {
        sprite: asset_server.load("big_log.png"),
        transform: Transform {
            translation: Vec3::new(-SPAWN_SHIFT, 9.5 * TILE_SIZE as f32, 0.),
            rotation: Quat::from_rotation_z(0.),
            scale: Vec3::ONE * LOG_SPRITE_SCALE,
        },
        timer: Timer::from_seconds(10.5, TimerMode::Repeating),
        velocity: Vec3::new(60., 0., 0.),
        size: Vec2::new(360., 70.) * LOG_SPRITE_SCALE,
    });
}

pub fn spawn_trunks(
    mut commands: Commands,
    mut trunk_spawner_query: Query<&mut TrunkSpawner>,
    time: Res<Time>,
) {
    for mut trunk_spawner in trunk_spawner_query.iter_mut() {
        debug!("{}", trunk_spawner.size);
        trunk_spawner.timer.tick(time.delta());
        if trunk_spawner.timer.just_finished() {
            commands.spawn((
                SpriteBundle {
                    transform: trunk_spawner.transform.clone(),
                    texture: trunk_spawner.sprite.clone(),
                    ..default()
                },
                Trunk,
                Velocity(trunk_spawner.velocity),
                SpriteSize(trunk_spawner.size),
            ));
        }
    }
}

pub fn despawn_trunks(
    mut commands: Commands,
    mut trunk_query: Query<(Entity, &Transform), With<Trunk>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (trunk_entity, trunk_transform) in trunk_query.iter_mut() {
        let trunk_pos = trunk_transform.translation;
        if trunk_pos.x < -DESPAWN_SHIFT || trunk_pos.x > window.width() + DESPAWN_SHIFT {
            commands.entity(trunk_entity).despawn();
        }
    }
}
