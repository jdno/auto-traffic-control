use std::ops::RangeInclusive;

use atc::v1::Map as ApiMap;

use crate::api::AsApi;
use crate::components::Tag;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

pub use self::airport::*;
pub use self::direction::*;
pub use self::node::Node;

mod airport;
mod direction;
mod node;

// TODO: Refactor constants below to ensure width and range always line up

/// The number of tiles that are left empty around the border of the window
const BORDER_SIZE: usize = 1;

/// The height of the map in tiles
pub const MAP_HEIGHT: usize = (SCREEN_HEIGHT as i32 / TILE_SIZE) as usize - (BORDER_SIZE * 2) - 1;

pub const MAP_HEIGHT_RANGE: RangeInclusive<i32> =
    -(MAP_HEIGHT as i32 / 2)..=(MAP_HEIGHT as i32 / 2);

/// The width of the map in tiles
pub const MAP_WIDTH: usize =
    ((SCREEN_WIDTH as i32 + TILE_SIZE) / TILE_SIZE) as usize - (BORDER_SIZE * 2) - 1;

pub const MAP_WIDTH_RANGE: RangeInclusive<i32> = -(MAP_WIDTH as i32 / 2)..=(MAP_WIDTH as i32 / 2);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Map {
    airports: Vec<Airport>,
    routing_grid: Vec<Node>,
}

impl Map {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn airports(&self) -> &[Airport] {
        &self.airports
    }

    pub fn routing_grid(&self) -> &Vec<Node> {
        &self.routing_grid
    }
}

impl Default for Map {
    fn default() -> Self {
        let airports = vec![
            Airport::new(Node::unrestricted(-2, -2), Direction::West, Tag::Red),
            Airport::new(Node::unrestricted(1, 4), Direction::South, Tag::Blue),
        ];

        let routing_grid = generate_routing_grid(&airports);

        Self {
            airports,
            routing_grid,
        }
    }
}

impl AsApi for Map {
    type ApiType = ApiMap;

    fn as_api(&self) -> Self::ApiType {
        ApiMap {
            airports: self
                .airports
                .iter()
                .map(|airport| airport.as_api())
                .collect(),
            routing_grid: self.routing_grid.iter().map(|node| node.as_api()).collect(),
            width: MAP_WIDTH as u32,
            height: MAP_HEIGHT as u32,
        }
    }
}

fn generate_routing_grid(airports: &[Airport]) -> Vec<Node> {
    let mut nodes = Vec::with_capacity(MAP_WIDTH * MAP_HEIGHT);

    for y in MAP_HEIGHT_RANGE {
        for x in MAP_WIDTH_RANGE {
            nodes.push(Node::unrestricted(x, y));
        }
    }

    for airport in airports {
        let airport_node = airport.node();

        for neighbor in airport_node.neighbors() {
            let direction_to_airport =
                Direction::between(&neighbor.as_point(), &airport_node.as_point());

            if direction_to_airport != airport.runway() {
                *nodes.get_mut(neighbor.as_index()).unwrap() =
                    Node::restricted(neighbor.longitude(), neighbor.latitude());
            }
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    use crate::map::{Node, MAP_HEIGHT, MAP_WIDTH};

    use super::Map;

    #[test]
    fn generate_routing_grid_removes_neighbors() {
        let map = Map::default();

        let airport = map.airports().get(0).unwrap().node();
        let neighbors = vec![
            Node::restricted(airport.longitude(), airport.latitude() + 1),
            Node::restricted(airport.longitude() + 1, airport.latitude() + 1),
            Node::restricted(airport.longitude() + 1, airport.latitude()),
            Node::restricted(airport.longitude() + 1, airport.latitude() - 1),
            Node::restricted(airport.longitude(), airport.latitude() - 1),
            Node::restricted(airport.longitude() - 1, airport.latitude() - 1),
            // Runway to the west
            Node::unrestricted(airport.longitude() - 1, airport.latitude()),
            Node::restricted(airport.longitude() - 1, airport.latitude() + 1),
        ];

        neighbors
            .iter()
            .for_each(|node| assert!(map.routing_grid().contains(node)));

        assert_eq!(MAP_WIDTH * MAP_HEIGHT, map.routing_grid().len());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Map>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Map>();
    }
}
