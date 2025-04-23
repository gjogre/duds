use bevy::{core_pipeline::bloom::Bloom, prelude::*};

mod asset_manager;
use asset_manager::{AssetManager, setup_asset_manager};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (setup_asset_manager, spawn_example_sprite, spawn_camera).chain(),
        )
        .run();
}

fn spawn_example_sprite(mut commands: Commands, asset_manager: Res<AssetManager>) {
    if let Some(sprite) = asset_manager.get_sprite(asset_manager::TileSheetType::Monsters, 2, 1) {
        commands.spawn((sprite, Transform::from_xyz(0.0, 0.0, 1.0)));
    } else {
        println!("Warning: Could not get sprite from asset manager");
    }
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: (500.),
            },

            scale: 1.,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
