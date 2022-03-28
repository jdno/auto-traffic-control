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

    let texture_handle = asset_server.load("sprites/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 8, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let horizontal_tiles = (SCREEN_WIDTH as i32 / TILE_SIZE + 1) as i32;
    let vertical_tiles = (SCREEN_HEIGHT as i32 / TILE_SIZE + 1) as i32;

    for y in -vertical_tiles..=vertical_tiles {
        for x in -horizontal_tiles..=horizontal_tiles {
            let x = (x * TILE_SIZE) as f32;
            let y = (y * TILE_SIZE) as f32;

            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, RenderLayer::Landscape.z()),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(0),
                ..Default::default()
            });

            // 25% chance of a decoration
            let sprite = match rng.gen_range(0..28) {
                0 => 1, // #
                1 => 2, // //
                2 => 3, // {
                3 => 4, // ;
                4 => 5, // )
                5 => 6, // &
                6 => 7, // }
                _ => continue,
            };

            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x - 16.0, y - 16.0, RenderLayer::Decoration.z()),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(sprite),
                ..Default::default()
            });
        }
    }
}
