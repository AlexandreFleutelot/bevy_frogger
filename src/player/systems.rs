use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

use super::components::Frog;
use crate::components::SpriteSize;
use crate::events::GameOverEvent;
use crate::{GRID_SIZE, TILE_SIZE, TOP_BAR};

use crate::trunks::components::Trunk;
use crate::turtles::components::Turtle;

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

pub fn player_fall_water(
    frog_query: Query<(&Frog, &SpriteSize, &Transform), With<Frog>>,
    trunk_query: Query<(&SpriteSize, &Transform), With<Trunk>>,
    turtle_query: Query<(&SpriteSize, &Transform), With<Turtle>>,
    mut ev_gameover: EventWriter<GameOverEvent>,
) {
    for (frog, frog_size, frog_transform) in frog_query.iter() {
        if frog.active {
            let frog_pos: Vec3 = frog_transform.translation;

            let over_water_area: bool =
                frog_pos.y > 7.0 * TILE_SIZE && frog_pos.y < 12.0 * TILE_SIZE;
            if over_water_area {
                let mut on_trunk: bool = false;
                for (trunk_size, trunk_transform) in trunk_query.iter() {
                    let trunk_pos: Vec3 = trunk_transform.translation;
                    let trunk_collision = collide(frog_pos, frog_size.0, trunk_pos, trunk_size.0);
                    if let Some(_collision) = trunk_collision {
                        on_trunk = true;
                    }
                }

                let mut on_turtle: bool = false;
                for (turtle_size, turtle_transform) in turtle_query.iter() {
                    let turtle_pos: Vec3 = turtle_transform.translation;
                    let turtle_collision =
                        collide(frog_pos, frog_size.0, turtle_pos, turtle_size.0);
                    if let Some(_collision) = turtle_collision {
                        on_turtle = true;
                    }
                }

                if !on_trunk && !on_turtle {
                    ev_gameover.send(GameOverEvent {
                        message: "frog has fall in the water".to_string(),
                    });
                }
            }
        }
    }
}
