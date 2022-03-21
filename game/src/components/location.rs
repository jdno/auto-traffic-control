use std::fmt::{Display, Formatter};

use bevy::prelude::*;
use geo::Point;

use atc::v1::Point as ApiPoint;

use crate::api::IntoApi;
use crate::map::Node;
use crate::TILE_SIZE;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Component)]
pub struct Location {
    x: i32,
    y: i32,
}

impl Location {
    #[cfg(test)]
    pub fn new(x: i32, y: i32) -> Self {
        Location { x, y }
    }

    #[allow(dead_code)] // TODO: Remove when the value is read
    pub fn x(&self) -> i32 {
        self.x
    }

    #[allow(dead_code)] // TODO: Remove when the value is read
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

impl From<&Node> for Location {
    fn from(tile: &Node) -> Self {
        Self {
            x: tile.longitude() * TILE_SIZE,
            y: tile.latitude() * TILE_SIZE,
        }
    }
}

impl From<&Mut<'_, bevy::prelude::Transform>> for Location {
    fn from(transform: &Mut<'_, bevy::prelude::Transform>) -> Self {
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
    type ApiType = ApiPoint;

    fn into_api(self) -> Self::ApiType {
        ApiPoint {
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
        let location = Location { x: 1, y: 2 };

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
