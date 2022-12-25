use std::time::Duration;

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, time::FixedTimestep};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const TIME_STEP: f32 = 1.0 / 120.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/background-day.png"),
        ..Default::default()
    });
}

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Wall;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
}

impl WallBundle {
    fn new(asset_server: Res<AssetServer>) -> WallBundle {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.9..1.2);
        let y = rng.gen_range(0.8..1.8);
        let flip = rng.gen_bool(0.5);
        let ty = if flip { 200.0 } else { -200.0 };
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(150.0, ty, 1.0),
                    scale: Vec3::new(x, y, 1.0),
                    ..default()
                },
                texture: asset_server.load("sprites/pipe-green.png"),
                sprite: Sprite {
                    flip_y: flip,
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn spawn_bird(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/bird.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 24.0), 1, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(-80.0, 0.0, 1.0)),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        },
        Bird,
        RigidBody::Dynamic,
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: -0.8,
        },
        Collider::ball(14.0),
        GravityScale(2.0),
        Sleeping::disabled(),
        ActiveEvents::COLLISION_EVENTS,
        AnimationTimer(Timer::from_seconds(0.14, TimerMode::Repeating)),
    ));
}

fn spawn_wall(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        WallBundle::new(asset_server),
        RigidBody::KinematicVelocityBased,
        Velocity {
            linvel: Vec2::new(-150.0, 0.0),
            angvel: 0.0,
        },
        Collider::cuboid(25.0, 155.0),
        Sensor::default(),
        Wall,
    ));
}

fn game_over(
    collision_events: EventReader<CollisionEvent>,
    mut entitys: Query<&mut RigidBody>,
    mut config: ResMut<WallSpawnConfig>,
) {
    if !collision_events.is_empty() {
        println!("Game Over!");
        for mut entity in entitys.iter_mut() {
            *entity = RigidBody::Fixed;
        }
        config.timer.pause();
    }
}

#[derive(Resource)]
struct WallSpawnConfig {
    timer: Timer,
}

fn spawn_wall_timer(
    commands: Commands,
    time: Res<Time>,
    mut config: ResMut<WallSpawnConfig>,
    asset_server: Res<AssetServer>,
) {
    config.timer.tick(time.delta());
    if config.timer.just_finished() {
        spawn_wall(commands, asset_server);
    }
}

fn setup_wall_spawning(mut commands: Commands) {
    commands.insert_resource(WallSpawnConfig {
        timer: Timer::new(Duration::from_millis(1700), TimerMode::Repeating),
    })
}

fn clean_wall(mut commands: Commands, mut query: Query<(Entity, &Transform), With<Wall>>) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.x < -180.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn move_bird(
    keyboard_input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Bird>>,
) {
    if mouse.just_pressed(MouseButton::Left) || keyboard_input.just_pressed(KeyCode::Space) {
        let (mut v, mut t) = query.single_mut();
        (*v).linvel = Vec2::new(0.0, 150.0);
        (*t).rotation = Quat::from_rotation_z(0.55);
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Velocity,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (_, mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn main() {
    App::new()
        .add_startup_system(spawn_bird)
        .add_startup_system(setup)
        .add_startup_system(spawn_wall)
        .add_startup_system(setup_wall_spawning)
        .add_system(move_bird)
        .add_system(spawn_wall_timer)
        .add_system(clean_wall)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(animate_sprite)
                // .with_system(game_over)
        )
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Flappy Bird!".to_string(),
                        canvas: Some("#flappy-bird".into()),
                        width: 288.0, // 288.0,
                        height: 512.0,
                        ..default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        // .add_plugin(WorldInspectorPlugin::new())
        .run();
}
