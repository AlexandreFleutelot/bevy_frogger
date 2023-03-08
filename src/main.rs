use std::f32::consts::PI;

use bevy::prelude::*;

const TILE_SIZE:f32 = 50.0;

enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    fn get_vect(&self) -> Vec3 {
        match self {
            Self::Left => Vec3 { x: -1.0, y: 0.0, z: 0.0 },
            Self::Right => Vec3 { x: 1.0, y: 0.0, z: 0.0 },
            Self::Up => Vec3 { x: 0.0, y: 1.0, z: 0.0 },
            Self::Down => Vec3 { x: 0.0, y: -1.0, z: 0.0 }
        }
    }
    fn get_quat(&self) -> Quat {
        match self {
            Self::Left => Quat::from_rotation_z(-PI/2.0),
            Self::Right => Quat::from_rotation_z(PI/2.0),
            Self::Up => Quat::from_rotation_z(PI),
            Self::Down => Quat::IDENTITY,
        }
    }
}

#[derive(Component)]
struct Frog {
    direction: Direction
}

enum AnimationState {
    Start,
    Run,
    Stop
}

#[derive(Component)]
struct Animation {
    frame_count: usize,
    timer: Timer,
    state: AnimationState
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_startup_system(setup)
        .add_startup_system(spawn_frog)
        .add_system(frog_controller)
        .run();
}



fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}


fn spawn_frog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let frog_texture = asset_server.load("frog.png");

    let texture_atlas_handle =  texture_atlases.add(TextureAtlas::from_grid(
        frog_texture, 
        Vec2::new(56.0, 80.0), 
        7, 1, 
        None, None));

    commands.spawn(
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform { 
                translation: Vec3::ZERO, 
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE },
            ..default()
        })
        .insert(Frog { direction: Direction::Up})
        .insert(Animation { 
            state: AnimationState::Stop,
            frame_count: 6, 
            timer: Timer::from_seconds(0.05, TimerMode::Repeating)});

}

fn frog_controller(
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut frog_query: Query<(&mut Transform, &mut Animation, &mut TextureAtlasSprite, &mut Frog)>,
) {

    for (mut frog_transform, mut animation, mut sprite, mut frog) in frog_query.iter_mut() {
        
        match animation.state {
            AnimationState::Start => {
                sprite.index = 0;
                animation.state = AnimationState::Run;
            },
            AnimationState::Run => {
                animation.timer.tick(time.delta());
                if animation.timer.just_finished() {
                    sprite.index += 1;
                    frog_transform.rotation = frog.direction.get_quat();
                    let dir = frog.direction.get_vect();
                    frog_transform.translation += dir * TILE_SIZE / (animation.frame_count as f32);
                    if sprite.index >= animation.frame_count { animation.state = AnimationState::Stop }
                }
            },
            AnimationState::Stop => {
                if kb.just_pressed(KeyCode::Right) { 
                    animation.state = AnimationState::Start;
                    frog.direction = Direction::Right;
                }else if kb.just_pressed(KeyCode::Left) { 
                    animation.state = AnimationState::Start;
                    frog.direction = Direction::Left;
                }else if kb.just_pressed(KeyCode::Up) { 
                    animation.state = AnimationState::Start;
                    frog.direction = Direction::Up;
                }else if kb.just_pressed(KeyCode::Down) { 
                    animation.state = AnimationState::Start;
                    frog.direction = Direction::Down;
                }
            },
        }
    }
}    