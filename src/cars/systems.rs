use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{TILE_SIZE};
use crate::components::Velocity;
use super::components::*;

pub fn set_car_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        CarSpawner {
            sprite: asset_server.load("blue_car.png"),
            transform: Transform { 
                translation: Vec3::new(0., 1.5*TILE_SIZE as f32, 0.), 
                rotation: Quat::from_rotation_z(0.), 
                scale: Vec3::ONE * 0.7 },
            timer: Timer::from_seconds(3.8, TimerMode::Repeating),
            velocity: Vec3::new(70.,0.,0.),
        }
    );

    commands.spawn(
        CarSpawner {
            sprite: asset_server.load("red_truck.png"),
            transform: Transform { 
                translation: Vec3::new(window.width(), 2.5*TILE_SIZE as f32, 0.), 
                rotation: Quat::from_rotation_z(PI), 
                scale: Vec3::ONE * 0.7 },
            timer: Timer::from_seconds(7.2, TimerMode::Repeating),
            velocity: Vec3::new(-50.,0.,0.),
        }
    );

    commands.spawn(
        CarSpawner {
            sprite: asset_server.load("yellow_car.png"),
            transform: Transform { 
                translation: Vec3::new(0., 3.5*TILE_SIZE as f32, 0.), 
                rotation: Quat::from_rotation_z(0.), 
                scale: Vec3::ONE * 0.7 },
            timer: Timer::from_seconds(5.1, TimerMode::Repeating),
            velocity: Vec3::new(60.,0.,0.),
        }
    );

    commands.spawn(
        CarSpawner {
            sprite: asset_server.load("green_car.png"),
            transform: Transform { 
                translation: Vec3::new(window.width(), 4.5*TILE_SIZE as f32, 0.), 
                rotation: Quat::from_rotation_z(PI), 
                scale: Vec3::ONE * 0.7 },
            timer: Timer::from_seconds(4.3, TimerMode::Repeating),
            velocity: Vec3::new(-90.,0.,0.),
        }
    );

    commands.spawn(
        CarSpawner {
            sprite: asset_server.load("brown_truck.png"),
            transform: Transform { 
                translation: Vec3::new(0., 5.5*TILE_SIZE as f32, 0.), 
                rotation: Quat::from_rotation_z(0.), 
                scale: Vec3::ONE * 0.7 },
            timer: Timer::from_seconds(4.7, TimerMode::Repeating),
            velocity: Vec3::new(75.,0.,0.),
        }
    );
}

pub fn spawn_cars(
    mut commands: Commands,
    mut car_spawner_query: Query<&mut CarSpawner>,
    time:Res<Time>
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
            Car
            ));            
        }
    }
}

