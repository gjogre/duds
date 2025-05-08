use bevy::prelude::*;
mod asset_manager;
mod components;
mod entities;
mod events;
mod game_ui;
mod systems;
use asset_manager::{
    attach_sprites, setup_asset_manager, slice_tilesheets_into_cache,
    sync_transform_to_map_position,
};
use events::HighlightEvent;
use game_ui::gameui::{button_system, setup_game_ui};
use systems::{
    game_input::cursor::*,
    pathfinding::*,
    tile_map::{generation::*, highlight::*},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    AssetLoading,
    Game,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Duds".into(),
                    resolution: (1280., 720.).into(),
                    ..default()
                }),
                exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
                ..default()
            }),
            MeshPickingPlugin,
        ))
        .add_event::<HighlightEvent>()
        .insert_state::<AppState>(AppState::AssetLoading)
        .add_systems(
            Startup,
            (setup_asset_manager, test_stuff, spawn_camera, setup_game_ui).chain(),
        )
        .add_systems(
            Update,
            (
                slice_tilesheets_into_cache.run_if(in_state(AppState::AssetLoading)),
                attach_sprites.run_if(in_state(AppState::Game)),
                cursor_clicked,
                highlight_changed,
                highlight_target_path,
                find_path,
                button_system,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                move_along_path,
                sync_transform_to_map_position,
                camera_movement_system,
            ),
        )
        .run();
}

fn test_stuff(mut commands: Commands) {
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
        components::tiles::Layer(1),
    ));
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
        ..default()
    });
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(16.0, 16.0, 20.0),
    ));
    generate_test_map(commands);
}
fn spawn_camera(mut commands: Commands) {
    // commands.spawn((
    //     Name::new("Camera"),
    //     Camera2d::default(),
    //     Projection::from(OrthographicProjection {
    //         scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
    //             viewport_height: (500.),
    //         },

    //         scale: 1.,
    //         ..OrthographicProjection::default_2d()
    //     }),
    //     Camera {
    //         order: 1,
    //         ..default()
    //     },
    //     Transform::from_xyz(256.0, 240.0, 5.0),
    // ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 10.0, 30.0).looking_at(Vec3::new(10.0, 10.0, 2.0), Vec3::Y),
        Camera { ..default() },
    ));
}

fn camera_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyQ) {
        direction += Vec3::Z;
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        direction -= Vec3::Z;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec3::Y;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= Vec3::Y;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= Vec3::X;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec3::X;
    }

    for mut transform in query.iter_mut() {
        transform.translation += direction * time.delta_secs() * 10.0;
    }
}
