use std::collections::HashMap;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension},
};

use crate::{
    AppState,
    components::{attributes::Moving, tiles::*},
    systems::tile_map::util::TILE_SIZE,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum TileSheetType {
    World,
    Monsters,
}

#[derive(Resource, Default)]
pub struct SpriteCache {
    pub sprites: HashMap<(TileSheetType, UVec2), Handle<Image>>,
}

#[derive(Resource)]
pub struct AssetManager {
    sheets: HashMap<TileSheetType, Handle<Image>>,
}

pub fn attach_sprites(
    mut commands: Commands,
    sprite_cache: Res<SpriteCache>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &SheetSprite, &MapPosition, Option<&Layer>), Without<Mesh3d>>,
) {
    for (entity, sheet_sprite, map_position, layer) in query.iter() {
        let index = UVec2::new(sheet_sprite.tilesheet_x, sheet_sprite.tilesheet_y);
        let key = (sheet_sprite.tilesheet.clone(), index);

        if let Some(image_handle) = sprite_cache.sprites.get(&key) {
            let cube_mesh = meshes.add(Cuboid {
                half_size: Vec3::splat(0.5),
                ..Default::default()
            });

            let material = materials.add(StandardMaterial {
                base_color_texture: Some(image_handle.clone()),
                ..Default::default()
            });

            commands.entity(entity).insert((
                Transform::from_xyz(
                    map_position.x as f32,
                    map_position.y as f32,
                    layer.map(|l| l.0 as f32).unwrap_or(0.0),
                ),
                Mesh3d(cube_mesh),
                MeshMaterial3d(material),
            ));
        } else {
            //warn!("Sprite not found in cache for {:?}", key);
        }
    }
}
pub fn sync_transform_to_map_position(
    time: Res<Time<Fixed>>,
    mut query: Query<(&MapPosition, &mut Transform, Option<&Moving>)>,
) {
    for (map_pos, mut transform, maybe_moving) in query.iter_mut() {
        let target_pos = Vec3::new(map_pos.x as f32, map_pos.y as f32, transform.translation.z);
        let diff = target_pos - transform.translation;

        if diff.length_squared() < 1.0 {
            transform.translation = target_pos;
            continue;
        }

        let speed = maybe_moving
            .map(|m| m.speed * TILE_SIZE as f32)
            .unwrap_or(100.0);

        transform.translation += diff.normalize() * speed * time.delta_secs();
    }
}

pub fn setup_asset_manager(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut asset_manager = AssetManager {
        sheets: HashMap::new(),
    };

    asset_manager.sheets.insert(
        TileSheetType::World,
        asset_server.load("tilesheets/tiny_dungeon_world.png"),
    );

    asset_manager.sheets.insert(
        TileSheetType::Monsters,
        asset_server.load("tilesheets/tiny_dungeon_monsters.png"),
    );

    commands.insert_resource(asset_manager);
    commands.insert_resource(SpriteCache::default());
}

pub fn slice_tilesheets_into_cache(
    mut images: ResMut<Assets<Image>>,
    mut sprite_cache: ResMut<SpriteCache>,
    asset_manager: Res<AssetManager>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    info!("slice_tilesheets_into_cache");

    for (sheet_type, handle) in &asset_manager.sheets {
        info!("handle and sheet: ({:?}, {:?})", sheet_type, handle);

        if let Some(image) = images.get(handle) {
            let tile_size_u32 = TILE_SIZE as u32;

            let columns = image.texture_descriptor.size.width / tile_size_u32;
            let rows = image.texture_descriptor.size.height / tile_size_u32;
            info!("colrows: ({:?}, {:?})", columns, rows);
            let cloned_image = image.clone();
            let texture_width = cloned_image.texture_descriptor.size.width;
            if let Some(data) = &cloned_image.data {
                let layout = image.texture_descriptor.format;

                for y in 0..rows {
                    for x in 0..columns {
                        let mut tile_data = Vec::new();

                        for row in 0..tile_size_u32 {
                            let src_row = y * tile_size_u32 + row;
                            let start = (src_row * texture_width + x * tile_size_u32) * 4;
                            let end = start + tile_size_u32 * 4;

                            tile_data.extend_from_slice(&data[start as usize..end as usize]);
                        }

                        let tile_image = Image::new(
                            Extent3d {
                                width: tile_size_u32,
                                height: tile_size_u32,
                                depth_or_array_layers: 1,
                            },
                            TextureDimension::D2,
                            tile_data,
                            layout,
                            RenderAssetUsages::RENDER_WORLD,
                        );

                        let handle = images.add(tile_image);
                        sprite_cache
                            .sprites
                            .insert((sheet_type.clone(), UVec2::new(y, x)), handle);
                        info!(
                            "Inserted sprite: ({:?}, {:?})",
                            sheet_type,
                            UVec2::new(x, y)
                        );
                    }
                }
            } else {
                warn!("Image data not loaded yet for sheet: {:?}", sheet_type);
                continue;
            }
        } else {
            warn!("Failed getting image data for sheet: {:?}", sheet_type);
            continue;
        }
    }

    next_state.set(AppState::Game);
    info!("Finished slicing all tilesheets.");
    //commands.remove_resource::<AssetManager>();
}
