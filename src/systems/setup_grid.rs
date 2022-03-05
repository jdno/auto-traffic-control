use std::ops::RangeInclusive;

use bevy::prelude::*;

use crate::map::{Coordinate, Tile};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

/// The number of tiles that are left empty around the border of the window
const BORDER_SIZE: usize = 1;

/// The height of the map in tiles
const MAP_HEIGHT: usize = (SCREEN_HEIGHT as i32 / TILE_SIZE) as usize - (BORDER_SIZE * 2) - 1;

const MAP_HEIGHT_RANGE: RangeInclusive<i32> = -(MAP_HEIGHT as i32 / 2)..=(MAP_HEIGHT as i32 / 2);

/// The width of the map in tiles
const MAP_WIDTH: usize = (SCREEN_WIDTH as i32 / TILE_SIZE) as usize - (BORDER_SIZE * 2) - 1;

const MAP_WIDTH_RANGE: RangeInclusive<i32> = -(MAP_WIDTH as i32 / 2)..=(MAP_WIDTH as i32 / 2);

pub fn setup_grid(mut commands: Commands) {
    for y in MAP_HEIGHT_RANGE {
        for x in MAP_WIDTH_RANGE {
            let tile = Tile::new(x, y);
            let coordinate = Coordinate::from(&tile);

            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(coordinate.x() as f32, coordinate.y() as f32, 0.0),
                    scale: Vec3::new(2.0, 2.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
