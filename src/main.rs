use bevy::prelude::*;
mod asset_manager;
mod components;
mod entities;
mod events;
mod systems;
use asset_manager::{attach_sprites, setup_asset_manager, sync_transform_to_map_position};
use events::HighlightEvent;
use systems::{
    game_input::cursor::*,
    pathfinding::*,
    tile_map::{generation::*, highlight::*, visualization::*},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Duds".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
            ..default()
        }))
        .insert_resource(CursorPosition {
            world: Vec2::ZERO,
            screen: Vec2::ZERO,
        })
        .add_event::<HighlightEvent>()
        .add_systems(
            Startup,
            (setup_asset_manager, spawn_example_sprite, spawn_camera).chain(),
        )
        .add_systems(
            Update,
            (
                attach_sprites,
                cursor_moved,
                cursor_events,
                cursor_clicked,
                highlight_mouse_hover,
                highlight_changed,
                visualize_target_path,
                find_path,
                sync_transform_to_map_position,
                move_along_path,
            ),
        )
        .run();
}

fn spawn_example_sprite(mut commands: Commands) {
    commands.spawn((
        components::basic::Player,
        components::tiles::Target {
            path: None,
            position: None,
        },
        components::tiles::SheetSprite {
            tilesheet: asset_manager::TileSheetType::Monsters,
            tilesheet_x: 2,
            tilesheet_y: 1,
        },
        components::tiles::MapPosition { x: 10, y: 10 },
        components::tiles::Layer(2),
    ));

    generate_test_map(commands);
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
