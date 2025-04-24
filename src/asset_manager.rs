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
