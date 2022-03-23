use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::rendering::RenderLayer;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

const TEXTURE_SIZE: i32 = 16;

pub fn setup_landscape(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = thread_rng();

    let texture_handle = asset_server.load("sprites/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let horizontal_tiles = SCREEN_WIDTH as i32 / TEXTURE_SIZE / 2 + 1;
    let vertical_tiles = SCREEN_HEIGHT as i32 / TEXTURE_SIZE / 2 + 1;

    for y in -vertical_tiles..=vertical_tiles {
        for x in -horizontal_tiles..=horizontal_tiles {
            // 25% chance of a decoration
            let sprite = match rng.gen_range(0..12) {
                0 => 1, // tree
                1 => 2, // trees
                2 => 3, // bush
                _ => 0, // grass
            };

            let x = x * TEXTURE_SIZE;
            let y = y * TEXTURE_SIZE;

            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32, y as f32, RenderLayer::Landscape.z()),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(sprite),
                ..Default::default()
            });
        }
    }
}
