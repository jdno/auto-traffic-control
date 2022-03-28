use bevy::prelude::*;

use crate::components::Tag;
use crate::map::{Direction, Map};
use crate::rendering::RenderLayer;
use crate::TILE_SIZE;

pub fn setup_airport(
    mut commands: Commands,
    map: Res<Map>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 8, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for airport in map.airports() {
        let airport_vec3 = airport.node().as_vec3(RenderLayer::Airport.z());
        let runway_vec3 = airport_vec3 + airport.runway().to_vec3() * Vec3::splat(TILE_SIZE as f32);

        let color_offset = match airport.tag() {
            Tag::Blue => 24,
            Tag::Red => 32,
        };

        let airport_offset = match airport.runway() {
            Direction::North => 0,
            Direction::East => 2,
            Direction::South => 4,
            Direction::West => 6,
            _ => panic!("diagonal airports are not supported"),
        };
        let runway_offset = airport_offset + 1;

        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: airport_vec3,
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(color_offset + airport_offset),
            ..Default::default()
        });

        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: runway_vec3,
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(color_offset + runway_offset),
            ..Default::default()
        });
    }
}
