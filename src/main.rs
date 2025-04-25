use bevy::prelude::*;
mod asset_manager;
mod components;
mod entities;
mod map;
mod systems;
use asset_manager::{AssetManager, setup_asset_manager};
use systems::game_input::CursorState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CursorState {
            world: Vec2::ZERO,
            screen: Vec2::ZERO,
        })
        .add_systems(
            Startup,
            (setup_asset_manager, spawn_example_sprite, spawn_camera).chain(),
        )
        .add_systems(
            Update,
            (
                systems::map::Map::attach_sprites,
                systems::game_input::cursor_moved,
                systems::game_input::cursor_events,
                systems::map::Map::highlight_sprite,
            ),
        )
        .run();
}

fn spawn_example_sprite(mut commands: Commands, asset_manager: Res<AssetManager>) {
    if let Some(sprite) = asset_manager.get_sprite(&asset_manager::TileSheetType::Monsters, 2, 1) {
        commands.spawn((sprite, Transform::from_xyz(128.0, 128.0, 1.0)));
    } else {
        println!("Warning: Could not get sprite from asset manager");
    }
    map::map::generate_test_map(commands);
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
        Transform::from_xyz(256.0, 240.0, 5.0),
    ));
}
