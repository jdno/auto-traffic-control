use bevy::prelude::*;

use crate::components::{Airplane, Movement};
use crate::map::{Coordinate, Direction, MAP_WIDTH_RANGE};
use crate::TILE_SIZE;

pub fn spawn_airplane(mut commands: Commands) {
    let spawn = Coordinate::new(TILE_SIZE * MAP_WIDTH_RANGE.start(), 0);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(spawn.x() as f32, spawn.y() as f32, 2.0),
                scale: Vec3::new(8.0, 8.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::BLUE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Airplane)
        .insert(Movement::new(32.0, Direction::East));
}
