use crate::{
    components::{attributes::Moving, tiles::*},
    systems::tile_map::util::{TILE_SIZE, map_to_world_coordinates},
};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct TilesheetInfo {
    pub name: String,
    pub path: String,
    pub columns: u32,
    pub rows: u32,
}

pub enum TileSheetType {
    World,
    Monsters,
}

pub fn attach_sprites(
    mut commands: Commands,
    asset_manager: Res<AssetManager>,
    query: Query<(Entity, &SheetSprite, &MapPosition, Option<&Layer>), Without<Sprite>>,
) {
    for (entity, sheet_sprite, map_position, layer) in query.iter() {
        if let Some(sprite) = asset_manager.get_sprite(
            &sheet_sprite.tilesheet,
            sheet_sprite.tilesheet_x,
            sheet_sprite.tilesheet_y,
        ) {
            let (x, y) = map_to_world_coordinates(map_position);
            // println!("Creating Sprite coordinates: ({}, {})", x, y);
            commands.entity(entity).insert((
                sprite,
                Transform::from_xyz(x as f32, y as f32, layer.map(|l| l.0 as f32).unwrap_or(0.0)),
            ));
        } else {
            warn!("Failed to get sprite for entity {:?}", entity);
        }
    }
}

pub fn sync_transform_to_map_position(
    time: Res<Time>,
    mut query: Query<(&MapPosition, &mut Transform, Option<&Moving>)>,
) {
    for (map_pos, mut transform, maybe_moving) in query.iter_mut() {
        let (x, y) = map_to_world_coordinates(map_pos);
        let target_pos = Vec3::new(x as f32, y as f32, transform.translation.z);

        let diff = target_pos - transform.translation;

        if diff.length_squared() < 0.01 {
            transform.translation = target_pos;
            continue;
        }

        let speed = maybe_moving
            .map(|m| m.speed * TILE_SIZE as f32)
            .unwrap_or(100.0);
        transform.translation += diff.normalize() * speed * time.delta_secs();
    }
}
#[derive(Resource)]
pub struct AssetManager {
    sheets: HashMap<String, (Handle<Image>, Handle<TextureAtlasLayout>, u32)>, // u32 = columns
}

impl AssetManager {
    pub fn get_sprite(&self, sheet_type: &TileSheetType, x: u32, y: u32) -> Option<Sprite> {
        let (image, layout, columns) = self.sheets.get(match sheet_type {
            TileSheetType::World => "world",
            TileSheetType::Monsters => "monsters",
        })?;
        let index = x * *columns + y;
        let texture_atlas = TextureAtlas {
            index: index as usize,
            layout: layout.clone(),
        };

        let sprite = Sprite::from_atlas_image(image.clone(), texture_atlas);
        Some(sprite)
    }
}

pub fn setup_asset_manager(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tilesheets = vec![
        TilesheetInfo {
            name: "world".to_string(),
            path: "tilesheets/tiny_dungeon_world.png".to_string(),
            columns: 16,
            rows: 19,
        },
        TilesheetInfo {
            name: "monsters".to_string(),
            path: "tilesheets/tiny_dungeon_monsters.png".to_string(),
            columns: 16,
            rows: 32,
        },
    ];

    let mut manager = AssetManager {
        sheets: HashMap::new(),
    };

    for sheet in tilesheets {
        let image: Handle<Image> = asset_server.load(sheet.path.clone());

        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(16, 16),
            sheet.columns,
            sheet.rows,
            None,
            Some(UVec2::new(1, 0)),
        );

        let layout_handle = atlas_layouts.add(layout);

        manager
            .sheets
            .insert(sheet.name.clone(), (image, layout_handle, sheet.columns));
    }

    commands.insert_resource(manager);
}
