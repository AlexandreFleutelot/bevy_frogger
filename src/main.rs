use bevy::prelude::*;

#[derive(Component)]
struct Frog;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_startup_system(setup)
        .add_startup_system(spawn_frog)
        .add_system(frog_jump)
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
            ..default()
        })
        .insert(Frog)
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
}

fn frog_jump(
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut frog_query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite), With<Frog>>,
) {
    for (mut timer, mut sprite) in &mut frog_query {
        if sprite.index < 6 {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index += 1;
            }
        }else if kb.just_pressed(KeyCode::Space) { sprite.index = 0 }
    }
}    