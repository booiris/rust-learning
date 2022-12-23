use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::prelude::random;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sprites/background-day.png"),
        ..Default::default()
    });
}

#[derive(Component)]
struct Player;

fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sprites/bluebird-midflap.png"),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Flappy Bird!".to_string(),
            width: 288.0,
            height: 512.0,
            ..default()
        })
        .add_startup_system(spawn_bird)
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .run();
}
