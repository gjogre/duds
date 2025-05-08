use crate::{
    components::{
        basic::{PathMarker, Player},
        tiles::{Highlight, MapPosition, Target},
    },
    events::HighlightEvent,
};
use bevy::{pbr::NotShadowCaster, prelude::*};

pub fn highlight_changed(
    mut commands: Commands,
    mut ev_highlight: EventReader<HighlightEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tile_query: Query<(Entity, &MapPosition, &MeshMaterial3d<StandardMaterial>)>,
    mut player_query: Query<&mut Target, With<Player>>,
) {
    let mut target_changed = false;
    let mut target_map_position = None;
    for ev in ev_highlight.read() {
        if let Ok((entity, map_position, material_wrapper)) = tile_query.get(ev.0) {
            if ev.1 == true {
                commands.entity(entity).insert(Highlight);
                target_changed = true;
                target_map_position = Some(*map_position);
                if let Some(material) = materials.get_mut(&material_wrapper.0) {
                    material.emissive = Color::linear_rgb(0.3, 0.3, 0.0).into();
                }
            }
            if ev.1 == false {
                commands.entity(entity).remove::<Highlight>();
                if let Some(material) = materials.get_mut(&material_wrapper.0) {
                    material.emissive = Color::BLACK.into();
                }
            }
        }
    }
    if target_changed {
        if let Ok(mut player_target) = player_query.single_mut() {
            player_target.position = target_map_position;
        }
    }
}
pub fn highlight_target_path(
    mut commands: Commands,
    target_query: Query<&Target, Changed<Target>>,
    marker_query: Query<Entity, With<PathMarker>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Remove old markers
    if !target_query.is_empty() {
        for entity in marker_query.iter() {
            commands.entity(entity).try_despawn();
        }
    }

    // Create shared mesh and material
    let marker_mesh = meshes.add(Mesh::from(Cuboid {
        half_size: Vec3::new(0.4, 0.4, 0.05),
    }));

    let marker_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.6, 1.0, 0.4),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    for target in target_query.iter() {
        if let Some(path) = &target.path {
            for pos in path {
                commands.spawn((
                    Transform::from_xyz(pos.x as f32, pos.y as f32, 1.0),
                    Visibility::Visible,
                    MeshMaterial3d(marker_material.clone()),
                    Mesh3d(marker_mesh.clone()),
                    GlobalTransform::default(),
                    InheritedVisibility::default(),
                    PathMarker,
                    NotShadowCaster, // Optional
                ));
            }
        }
    }
}
