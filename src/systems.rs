use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::*;
use crate::events::GameOverEvent;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let background_image = asset_server.load("background.png");
    commands.spawn(SpriteBundle {
        texture: background_image,
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -10.0),
        ..default()
    });
}

pub fn move_items(mut movable_query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in movable_query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

pub fn game_over(mut ev_game_over: EventReader<GameOverEvent>) {
    for ev in ev_game_over.iter() {
        info!("{}", ev.message);
    }
}
