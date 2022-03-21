use std::cmp::{max, min};
use std::fmt::{Display, Formatter};

use bevy::prelude::*;
use geo::{point, Point};

use atc::v1::Node as ApiNode;

use crate::api::IntoApi;
use crate::map::{MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};
use crate::TILE_SIZE;

/// A tile in the game
///
/// Tiles divide the game world into regular, square fields. They are used to render the map, and to
/// create a routing grid on top of it.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Tile {
    x: i32,
    y: i32,
}

impl Tile {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn neighbors(&self) -> Vec<Tile> {
        let width_range =
            max(*MAP_WIDTH_RANGE.start(), self.x - 1)..=min(*MAP_WIDTH_RANGE.end(), self.x + 1);
        let height_range =
            max(*MAP_HEIGHT_RANGE.start(), self.y - 1)..=min(*MAP_HEIGHT_RANGE.end(), self.y + 1);

        let mut neighbors = Vec::new();

        for y in height_range {
            for x in width_range.clone() {
                // TODO: Refactor to avoid clone
                if x == self.x && y == self.y {
                    continue;
                }
                neighbors.push(Tile::new(x, y));
            }
        }

        neighbors
    }

    pub fn is_neighbor(&self, potential_neighbor: &Tile) -> bool {
        let delta_x = potential_neighbor.x() - self.x();
        let delta_y = potential_neighbor.y() - self.y();

        delta_x.abs() <= 1 && delta_y.abs() <= 1
    }

    pub fn as_point(&self) -> Point<f32> {
        let x = (self.x * TILE_SIZE) as f32;
        let y = (self.y * TILE_SIZE) as f32;

        point!(x: x, y: y)
    }

    pub fn as_vec3(&self, z: f32) -> Vec3 {
        Vec3::new((self.x * TILE_SIZE) as f32, (self.y * TILE_SIZE) as f32, z)
    }

    pub fn to_location(self) -> (i32, i32) {
        (self.x * TILE_SIZE, self.y * TILE_SIZE)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl From<&Point<i32>> for Tile {
    fn from(point: &Point<i32>) -> Self {
        let x = point.x() / TILE_SIZE;
        let y = point.y() / TILE_SIZE;

        Self { x, y }
    }
}

impl IntoApi for Tile {
    type ApiType = ApiNode;

    fn into_api(self) -> Self::ApiType {
        ApiNode {
            longitude: self.x(),
            latitude: self.y(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};
    use geo::point;

    use super::{Tile, TILE_SIZE};

    #[test]
    fn neighbors_at_center() {
        let tile = Tile::new(0, 0);

        let neighbors = tile.neighbors();

        assert_eq!(
            vec![
                Tile::new(-1, -1),
                Tile::new(0, -1),
                Tile::new(1, -1),
                Tile::new(-1, 0),
                Tile::new(1, 0),
                Tile::new(-1, 1),
                Tile::new(0, 1),
                Tile::new(1, 1),
            ],
            neighbors
        );
    }

    #[test]
    fn neighbors_at_edge() {
        let edge = *MAP_WIDTH_RANGE.start();
        let tile = Tile::new(edge, 0);

        let neighbors = tile.neighbors();

        assert_eq!(
            vec![
                Tile::new(edge, -1),
                Tile::new(edge + 1, -1),
                Tile::new(edge + 1, 0),
                Tile::new(edge, 1),
                Tile::new(edge + 1, 1),
            ],
            neighbors
        );
    }

    #[test]
    fn neighbors_at_corner() {
        let x = *MAP_WIDTH_RANGE.start();
        let y = *MAP_HEIGHT_RANGE.start();

        let tile = Tile::new(x, y);

        let neighbors = tile.neighbors();

        assert_eq!(
            vec![
                Tile::new(x + 1, y),
                Tile::new(x, y + 1),
                Tile::new(x + 1, y + 1),
            ],
            neighbors
        );
    }

    #[test]
    fn is_neighbor_with_neighbor() {
        let tile = Tile::new(0, 0);
        let neighbor = Tile::new(1, 1);

        assert!(neighbor.is_neighbor(&tile));
    }

    #[test]
    fn is_neighbor_with_distant_tile() {
        let tile = Tile::new(0, 0);
        let neighbor = Tile::new(2, 0);

        assert!(!neighbor.is_neighbor(&tile));
    }

    #[test]
    fn trait_display() {
        let tile = Tile::new(1, 2);

        assert_eq!("Tile { x: 1, y: 2 }", &tile.to_string());
    }

    #[test]
    fn trait_from_0_point() {
        let point = point!(x: 0, y: 0);

        let tile = Tile::from(&point);

        assert_eq!(0, tile.x);
        assert_eq!(0, tile.y);
    }

    #[test]
    fn trait_from_point_smaller_than_tile_size() {
        let point = point!(x: TILE_SIZE / 2, y: TILE_SIZE / 2);

        let tile = Tile::from(&point);

        assert_eq!(0, tile.x);
        assert_eq!(0, tile.y);
    }

    #[test]
    fn trait_from_point_greater_than_tile_size() {
        let point = point!(x: TILE_SIZE * 2, y: TILE_SIZE * 3);

        let tile = Tile::from(&point);

        assert_eq!(2, tile.x);
        assert_eq!(3, tile.y);
    }

    #[test]
    fn trait_from_negative_point() {
        let point = point!(x: TILE_SIZE * -2, y: TILE_SIZE * -3);

        let tile = Tile::from(&point);

        assert_eq!(-2, tile.x);
        assert_eq!(-3, tile.y);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Tile>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Tile>();
    }
}
