use std::cmp::{max, min};
use std::fmt::{Display, Formatter};

use bevy::prelude::*;
use geo::{point, Point};

use atc::v1::Node as ApiNode;

use crate::api::AsApi;
use crate::map::{MAP_HEIGHT, MAP_HEIGHT_RANGE, MAP_WIDTH, MAP_WIDTH_RANGE};
use crate::TILE_SIZE;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Node {
    longitude: i32,
    latitude: i32,
    restricted: bool,
}

impl Node {
    pub fn restricted(longitude: i32, latitude: i32) -> Self {
        Self {
            longitude,
            latitude,
            restricted: true,
        }
    }

    pub fn unrestricted(longitude: i32, latitude: i32) -> Self {
        Self {
            longitude,
            latitude,
            restricted: false,
        }
    }

    pub fn longitude(&self) -> i32 {
        self.longitude
    }

    pub fn latitude(&self) -> i32 {
        self.latitude
    }

    pub fn is_restricted(&self) -> bool {
        self.restricted
    }

    // TODO: Move to map so that `restricted` can be set from the routing grid
    pub fn neighbors(&self) -> Vec<Node> {
        let width_range = max(*MAP_WIDTH_RANGE.start(), self.longitude - 1)
            ..=min(*MAP_WIDTH_RANGE.end(), self.longitude + 1);
        let height_range = max(*MAP_HEIGHT_RANGE.start(), self.latitude - 1)
            ..=min(*MAP_HEIGHT_RANGE.end(), self.latitude + 1);

        let mut neighbors = Vec::new();

        for y in height_range {
            // TODO: Refactor to avoid clone
            for x in width_range.clone() {
                if x == self.longitude && y == self.latitude {
                    continue;
                }

                // TODO: Get restriction from routing grid
                neighbors.push(Node::unrestricted(x, y));
            }
        }

        neighbors
    }

    pub fn is_neighbor(&self, potential_neighbor: &Node) -> bool {
        let delta_x = potential_neighbor.longitude() - self.longitude();
        let delta_y = potential_neighbor.latitude() - self.latitude();

        delta_x.abs() <= 1 && delta_y.abs() <= 1
    }

    pub fn as_index(&self) -> usize {
        let x = self.longitude + MAP_WIDTH as i32 / 2;
        let y = self.latitude + MAP_HEIGHT as i32 / 2;

        let index = (y * MAP_WIDTH as i32) + x;

        index as usize
    }

    pub fn as_point(&self) -> Point<f32> {
        let x = (self.longitude * TILE_SIZE) as f32;
        let y = (self.latitude * TILE_SIZE) as f32;

        point!(x: x, y: y)
    }

    pub fn as_vec3(&self, z: f32) -> Vec3 {
        Vec3::new(
            (self.longitude * TILE_SIZE) as f32,
            (self.latitude * TILE_SIZE) as f32,
            z,
        )
    }

    pub fn to_location(self) -> (i32, i32) {
        (self.longitude * TILE_SIZE, self.latitude * TILE_SIZE)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ x: {}, y: {} }}", self.longitude, self.latitude)
    }
}

impl From<&Point<i32>> for Node {
    fn from(point: &Point<i32>) -> Self {
        let x = point.x() / TILE_SIZE;
        let y = point.y() / TILE_SIZE;

        Self {
            longitude: x,
            latitude: y,
            restricted: false, // TODO: Get restriction from routing grid
        }
    }
}

impl AsApi for Node {
    type ApiType = ApiNode;

    fn as_api(&self) -> Self::ApiType {
        ApiNode {
            longitude: self.longitude(),
            latitude: self.latitude(),
            restricted: self.restricted,
        }
    }
}

#[cfg(test)]
mod tests {
    use geo::point;

    use crate::map::{MAP_HEIGHT, MAP_HEIGHT_RANGE, MAP_WIDTH, MAP_WIDTH_RANGE};

    use super::{Node, TILE_SIZE};

    #[test]
    fn neighbors_at_center() {
        let node = Node::unrestricted(0, 0);

        let neighbors = node.neighbors();

        assert_eq!(
            vec![
                Node::unrestricted(-1, -1),
                Node::unrestricted(0, -1),
                Node::unrestricted(1, -1),
                Node::unrestricted(-1, 0),
                Node::unrestricted(1, 0),
                Node::unrestricted(-1, 1),
                Node::unrestricted(0, 1),
                Node::unrestricted(1, 1),
            ],
            neighbors
        );
    }

    #[test]
    fn neighbors_at_edge() {
        let edge = *MAP_WIDTH_RANGE.start();
        let node = Node::unrestricted(edge, 0);

        let neighbors = node.neighbors();

        assert_eq!(
            vec![
                Node::unrestricted(edge, -1),
                Node::unrestricted(edge + 1, -1),
                Node::unrestricted(edge + 1, 0),
                Node::unrestricted(edge, 1),
                Node::unrestricted(edge + 1, 1),
            ],
            neighbors
        );
    }

    #[test]
    fn neighbors_at_corner() {
        let x = *MAP_WIDTH_RANGE.start();
        let y = *MAP_HEIGHT_RANGE.start();

        let node = Node::unrestricted(x, y);

        let neighbors = node.neighbors();

        assert_eq!(
            vec![
                Node::unrestricted(x + 1, y),
                Node::unrestricted(x, y + 1),
                Node::unrestricted(x + 1, y + 1),
            ],
            neighbors
        );
    }

    #[test]
    fn is_neighbor_with_neighbor() {
        let node = Node::unrestricted(0, 0);
        let neighbor = Node::unrestricted(1, 1);

        assert!(neighbor.is_neighbor(&node));
    }

    #[test]
    fn is_neighbor_with_distant_node() {
        let node = Node::unrestricted(0, 0);
        let neighbor = Node::unrestricted(2, 0);

        assert!(!neighbor.is_neighbor(&node));
    }

    #[test]
    fn as_index_at_bottom_left() {
        let node = Node::unrestricted(*MAP_WIDTH_RANGE.start(), *MAP_HEIGHT_RANGE.start());
        assert_eq!(0, node.as_index());
    }

    #[test]
    fn as_index_at_top_right() {
        let node = Node::unrestricted(*MAP_WIDTH_RANGE.end(), *MAP_HEIGHT_RANGE.end());
        assert_eq!((MAP_WIDTH * MAP_HEIGHT) - 1, node.as_index());
    }

    #[test]
    fn trait_display() {
        let node = Node::unrestricted(1, 2);

        assert_eq!("Node { x: 1, y: 2 }", &node.to_string());
    }

    #[test]
    fn trait_from_0_point() {
        let point = point!(x: 0, y: 0);

        let node = Node::from(&point);

        assert_eq!(0, node.longitude);
        assert_eq!(0, node.latitude);
    }

    #[test]
    fn trait_from_point_smaller_than_node_size() {
        let point = point!(x: TILE_SIZE / 2, y: TILE_SIZE / 2);

        let node = Node::from(&point);

        assert_eq!(0, node.longitude);
        assert_eq!(0, node.latitude);
    }

    #[test]
    fn trait_from_point_greater_than_node_size() {
        let point = point!(x: TILE_SIZE * 2, y: TILE_SIZE * 3);

        let node = Node::from(&point);

        assert_eq!(2, node.longitude);
        assert_eq!(3, node.latitude);
    }

    #[test]
    fn trait_from_negative_point() {
        let point = point!(x: TILE_SIZE * -2, y: TILE_SIZE * -3);

        let node = Node::from(&point);

        assert_eq!(-2, node.longitude);
        assert_eq!(-3, node.latitude);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Node>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Node>();
    }
}
