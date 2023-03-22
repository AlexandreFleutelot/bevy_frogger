use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::{SpriteSize, Velocity};
use crate::{DESPAWN_SHIFT, SPAWN_SHIFT, TILE_SIZE};

use super::components::*;

use super::TURTLE_SPRITE_SCALE;

pub fn set_turtle_spawner(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {

    let window = window_query.get_single().unwrap();

    commands.spawn(TurtleSpawner {
        sprite: asset_server.load("turtle.png"),
        transform: Transform {
            translation: Vec3::new(window.width() + SPAWN_SHIFT, 10.5 * TILE_SIZE as f32, 0.),
            rotation: Quat::from_rotation_z(PI),
            scale: Vec3::ONE * TURTLE_SPRITE_SCALE,
        },
        timer: Timer::from_seconds(7.1, TimerMode::Repeating),
        velocity: Vec3::new(-52., 0., 0.),
        size: Vec2::new(70., 70.) * TURTLE_SPRITE_SCALE,
    });

    commands.spawn(TurtleSpawner {
        sprite: asset_server.load("turtle.png"),
        transform: Transform {
            translation: Vec3::new(window.width() + SPAWN_SHIFT, 7.5 * TILE_SIZE as f32, 0.),
            rotation: Quat::from_rotation_z(PI),
            scale: Vec3::ONE * TURTLE_SPRITE_SCALE,
        },
        timer: Timer::from_seconds(6.5, TimerMode::Repeating),
        velocity: Vec3::new(-38., 0., 0.),
        size: Vec2::new(70., 70.) * TURTLE_SPRITE_SCALE,
    });
}

pub fn spawn_turtles(
    mut commands: Commands,
    mut turtle_spawner_query: Query<&mut TurtleSpawner>,
    time: Res<Time>,
) {
    for mut turtle_spawner in turtle_spawner_query.iter_mut() {
        debug!("{}", turtle_spawner.size);
        turtle_spawner.timer.tick(time.delta());
        if turtle_spawner.timer.just_finished() {
            for i in 0..3 {
                let mut transform =  turtle_spawner.transform.clone();
                transform.translation.x += turtle_spawner.size.x * i as f32;
                info!("{}",transform.translation);
                commands.spawn((
                    SpriteBundle {
                        transform: transform,
                        texture: turtle_spawner.sprite.clone(),
                        ..default()
                    },
                    Turtle,
                    Velocity(turtle_spawner.velocity),
                    SpriteSize(turtle_spawner.size),
                ));
            }
            
        }
    }
}

pub fn despawn_turtles(
    mut commands: Commands,
    mut turtle_query: Query<(Entity, &Transform), With<Turtle>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (turtle_entity, turtle_transform) in turtle_query.iter_mut() {
        let turtle_pos = turtle_transform.translation;
        if turtle_pos.x < -DESPAWN_SHIFT || turtle_pos.x > window.width() + DESPAWN_SHIFT {
            commands.entity(turtle_entity).despawn();
        }
    }
}
