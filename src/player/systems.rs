use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

use super::components::Frog;
use crate::{components::SpriteSize, GRID_SIZE, TILE_SIZE, TOP_BAR};

pub fn spawn_frog(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("simple_frog.png"),
            transform: Transform {
                translation: Vec3 {
                    //bottom middle
                    x: GRID_SIZE.0 * TILE_SIZE / 2.0,
                    y: TILE_SIZE / 2.0,
                    z: 0.0,
                },
                rotation: Quat::from_rotation_z(PI), //facing up
                scale: Vec3::ONE,
            },
            ..default()
        },
        Frog { active: true },
        SpriteSize(Vec2::new(50., 50.)),
    ));
}

pub fn frog_controller(kb: Res<Input<KeyCode>>, mut frog_query: Query<(&Frog, &mut Transform)>) {
    for (frog, mut frog_transform) in frog_query.iter_mut() {
        if frog.active {
            if kb.just_pressed(KeyCode::Right) {
                frog_transform.translation.x += TILE_SIZE;
                frog_transform.rotation = Quat::from_rotation_z(PI / 2.0);
            } else if kb.just_pressed(KeyCode::Left) {
                frog_transform.translation.x -= TILE_SIZE;
                frog_transform.rotation = Quat::from_rotation_z(-PI / 2.0);
            } else if kb.just_pressed(KeyCode::Up) {
                frog_transform.translation.y += TILE_SIZE;
                frog_transform.rotation = Quat::from_rotation_z(PI);
            } else if kb.just_pressed(KeyCode::Down) {
                frog_transform.translation.y -= TILE_SIZE;
                frog_transform.rotation = Quat::IDENTITY;
            }
        }
    }
}

pub fn confine_player(
    mut frog_query: Query<&mut Transform, With<Frog>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for mut frog_transform in frog_query.iter_mut() {
        if frog_transform.translation.x < 0.0 {
            frog_transform.translation.x += TILE_SIZE;
        }
        if frog_transform.translation.x > window.width() {
            frog_transform.translation.x -= TILE_SIZE;
        }
        if frog_transform.translation.y < 0.0 {
            frog_transform.translation.y += TILE_SIZE;
        }
        if frog_transform.translation.y > window.height() - TOP_BAR {
            frog_transform.translation.y -= TILE_SIZE;
        }
    }
}
