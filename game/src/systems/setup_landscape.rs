use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::rendering::RenderLayer;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

pub fn setup_landscape(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = thread_rng();

    let landscape_handle = asset_server.load("sprites/landscape.png");
    let landscape_atlas = TextureAtlas::from_grid(landscape_handle, Vec2::new(32.0, 32.0), 1, 1);
    let landscape_atlas_handle = texture_atlases.add(landscape_atlas);

    let decorations_handle = asset_server.load("sprites/decorations.png");
    let decorations_atlas =
        TextureAtlas::from_grid(decorations_handle, Vec2::new(128.0, 128.0), 4, 2);
    let decorations_atlas_handle = texture_atlases.add(decorations_atlas);

    let horizontal_tiles = (SCREEN_WIDTH as i32 / TILE_SIZE + 1) as i32;
    let vertical_tiles = (SCREEN_HEIGHT as i32 / TILE_SIZE + 1) as i32;

    for y in -vertical_tiles..=vertical_tiles {
        for x in -horizontal_tiles..=horizontal_tiles {
            let x = (x * TILE_SIZE) as f32;
            let y = (y * TILE_SIZE) as f32;

            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: landscape_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, RenderLayer::Landscape.z()),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(0),
                ..Default::default()
            });

            // 25% chance of a decoration
            let sprite = rng.gen_range(0..32);
            if sprite >= 8 {
                continue;
            }

            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: decorations_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x - 16.0, y - 16.0, RenderLayer::Decoration.z()),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: sprite,
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
