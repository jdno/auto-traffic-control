use std::fmt::{Display, Formatter};

use bevy::prelude::*;
use geo::Point;

use atc::v1::Location as ApiLocation;

use crate::api::IntoApi;
use crate::map::Tile;
use crate::TILE_SIZE;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Location {
    x: i32,
    y: i32,
}

impl Location {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

impl From<&Point<f32>> for Location {
    fn from(point: &Point<f32>) -> Self {
        Self {
            x: point.x() as i32,
            y: point.y() as i32,
        }
    }
}

impl From<&Tile> for Location {
    fn from(tile: &Tile) -> Self {
        Self {
            x: tile.x() * TILE_SIZE,
            y: tile.y() * TILE_SIZE,
        }
    }
}

impl From<Mut<'_, bevy::prelude::Transform>> for Location {
    fn from(transform: Mut<'_, bevy::prelude::Transform>) -> Self {
        Self {
            x: transform.translation.x as i32,
            y: transform.translation.y as i32,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl IntoApi for Location {
    type ApiType = ApiLocation;

    fn into_api(self) -> Self::ApiType {
        ApiLocation {
            x: self.x,
            y: self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Location;

    #[test]
    fn trait_display() {
        let location = Location::new(1, 2);

        assert_eq!("Location { x: 1, y: 2 }", &location.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Location>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Location>();
    }
}
