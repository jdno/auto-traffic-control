use crate::components::Airplane;
use crate::map::MAP_WIDTH_RANGE;
use bevy::prelude::*;

use crate::TILE_SIZE;

pub fn spawn_airplane(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new((TILE_SIZE * MAP_WIDTH_RANGE.start()) as f32, 0.0, 2.0),
                scale: Vec3::new(8.0, 8.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::BLUE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Airplane);
}
