use std::ops::RangeInclusive;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

pub use self::direction::*;
pub use self::node::Node;

mod direction;
mod node;

/// The number of tiles that are left empty around the border of the window
const BORDER_SIZE: usize = 1;

/// The height of the map in tiles
pub const MAP_HEIGHT: usize = (SCREEN_HEIGHT as i32 / TILE_SIZE) as usize - (BORDER_SIZE * 2) - 1;

pub const MAP_HEIGHT_RANGE: RangeInclusive<i32> =
    -(MAP_HEIGHT as i32 / 2)..=(MAP_HEIGHT as i32 / 2);

/// The width of the map in tiles
pub const MAP_WIDTH: usize = (SCREEN_WIDTH as i32 / TILE_SIZE) as usize - (BORDER_SIZE * 2) - 1;

pub const MAP_WIDTH_RANGE: RangeInclusive<i32> = -(MAP_WIDTH as i32 / 2)..=(MAP_WIDTH as i32 / 2);
