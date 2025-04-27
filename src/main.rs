use bevy::prelude::*;
mod asset_manager;
mod components;
mod entities;
mod events;
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
    commands.spawn((
        components::player::Player,
        components::sheetsprite::SheetSprite {
            tilesheet: asset_manager::TileSheetType::Monsters,
            tilesheet_x: 2,
            tilesheet_y: 1,
        },
        components::map_position::MapPosition { x: 10, y: 10 },
        components::layer::Layer(1),
    ));

    systems::map::Map::generate_test_map(commands);
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
