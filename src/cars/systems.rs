use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

use super::{components::*, DESPAWN_SHIFT, SPAWN_SHIFT};
use crate::components::{SpriteSize, Velocity};
use crate::events::GameOverEvent;
use crate::player::components::Frog;
use crate::TILE_SIZE;

pub fn set_car_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(CarSpawner {
        sprite: asset_server.load("blue_car.png"),
        transform: Transform {
            translation: Vec3::new(-SPAWN_SHIFT, 1.5 * TILE_SIZE as f32, 0.),
            rotation: Quat::from_rotation_z(0.),
            scale: Vec3::ONE * 0.7,
        },
        timer: Timer::from_seconds(3.8, TimerMode::Repeating),
        velocity: Vec3::new(70., 0., 0.),
    });

    commands.spawn(CarSpawner {
        sprite: asset_server.load("red_truck.png"),
        transform: Transform {
            translation: Vec3::new(window.width() + SPAWN_SHIFT, 2.5 * TILE_SIZE as f32, 0.),
            rotation: Quat::from_rotation_z(PI),
            scale: Vec3::ONE * 0.7,
        },
        timer: Timer::from_seconds(7.2, TimerMode::Repeating),
        velocity: Vec3::new(-50., 0., 0.),
    });

    commands.spawn(CarSpawner {
        sprite: asset_server.load("yellow_car.png"),
        transform: Transform {
            translation: Vec3::new(-SPAWN_SHIFT, 3.5 * TILE_SIZE as f32, 0.),
            rotation: Quat::from_rotation_z(0.),
            scale: Vec3::ONE * 0.7,
        },
        timer: Timer::from_seconds(5.1, TimerMode::Repeating),
        velocity: Vec3::new(60., 0., 0.),
    });

    commands.spawn(CarSpawner {
        sprite: asset_server.load("green_car.png"),
        transform: Transform {
            translation: Vec3::new(window.width() + SPAWN_SHIFT, 4.5 * TILE_SIZE as f32, 0.),
            rotation: Quat::from_rotation_z(PI),
            scale: Vec3::ONE * 0.7,
        },
        timer: Timer::from_seconds(4.3, TimerMode::Repeating),
        velocity: Vec3::new(-90., 0., 0.),
    });

    commands.spawn(CarSpawner {
        sprite: asset_server.load("brown_truck.png"),
        transform: Transform {
            translation: Vec3::new(-SPAWN_SHIFT, 5.5 * TILE_SIZE as f32, 0.),
            rotation: Quat::from_rotation_z(0.),
            scale: Vec3::ONE * 0.7,
        },
        timer: Timer::from_seconds(4.7, TimerMode::Repeating),
        velocity: Vec3::new(75., 0., 0.),
    });
}

pub fn spawn_cars(
    mut commands: Commands,
    mut car_spawner_query: Query<&mut CarSpawner>,
    time: Res<Time>,
) {
    for mut car_spawner in car_spawner_query.iter_mut() {
        car_spawner.timer.tick(time.delta());
        if car_spawner.timer.just_finished() {
            commands.spawn((
                SpriteBundle {
                    transform: car_spawner.transform.clone(),
                    texture: car_spawner.sprite.clone(),
                    ..default()
                },
                Velocity(car_spawner.velocity),
                Car,
                SpriteSize(Vec2::new(50., 50.)),
            ));
        }
    }
}

pub fn despawn_cars(
    mut commands: Commands,
    mut car_query: Query<(Entity, &Transform), With<Car>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (car_entity, car_transform) in car_query.iter_mut() {
        let car_pos = car_transform.translation;
        if car_pos.x < -DESPAWN_SHIFT || car_pos.x > window.width() + DESPAWN_SHIFT {
            commands.entity(car_entity).despawn();
        }
    }
}

pub fn car_frog_collision(
    frog_query: Query<(&Frog, &SpriteSize, &Transform), With<Frog>>,
    car_query: Query<(&SpriteSize, &Transform), With<Car>>,
    mut ev_gameover: EventWriter<GameOverEvent>,
) {
    for (frog, frog_size, frog_transform) in frog_query.iter() {
        if frog.active {
            let frog_pos = frog_transform.translation;

            for (car_size, car_transform) in car_query.iter() {
                let car_pos = car_transform.translation;

                let collision = collide(frog_pos, frog_size.0, car_pos, car_size.0);
                if let Some(_collision) = collision {
                    ev_gameover.send(GameOverEvent {
                        message: "Hit by car".to_string(),
                    });
                }
            }
        }
    }
}
