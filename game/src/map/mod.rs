use std::ops::RangeInclusive;

use atc::v1::Map as ApiMap;

use crate::api::AsApi;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

pub use self::direction::*;
pub use self::node::Node;

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
    airport: Node,
    routing_grid: Vec<Node>,
}

impl Map {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn airport(&self) -> &Node {
        &self.airport
    }

    pub fn routing_grid(&self) -> &Vec<Node> {
        &self.routing_grid
    }
}

impl Default for Map {
    fn default() -> Self {
        let airport = Node::new(0, 0);
        let routing_grid = generate_routing_grid(&airport);

        Self {
            airport,
            routing_grid,
        }
    }
}

impl AsApi for Map {
    type ApiType = ApiMap;

    fn as_api(&self) -> Self::ApiType {
        ApiMap {
            airport: Some(self.airport.as_api()),
            routing_grid: self.routing_grid.iter().map(|node| node.as_api()).collect(),
        }
    }
}

fn generate_routing_grid(airport: &Node) -> Vec<Node> {
    let direction_of_runway = Direction::West;

    let mut nodes = Vec::with_capacity(MAP_WIDTH * MAP_HEIGHT - 7);

    for y in MAP_HEIGHT_RANGE {
        for x in MAP_WIDTH_RANGE {
            let node = Node::new(x, y);
            let direction_to_airport = Direction::between(&node.as_point(), &airport.as_point());

            if airport != &node
                && airport.is_neighbor(&node)
                && direction_to_airport != direction_of_runway
            {
                continue;
            }

            nodes.push(node);
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::Map;
    use crate::map::{Node, MAP_HEIGHT, MAP_WIDTH};

    #[test]
    fn generate_routing_grid_removes_neighbors() {
        let map = Map::default();

        let airport = map.airport();
        let neighbors = vec![
            Node::new(airport.longitude(), airport.latitude() + 1),
            Node::new(airport.longitude() + 1, airport.latitude() + 1),
            Node::new(airport.longitude() + 1, airport.latitude()),
            Node::new(airport.longitude() + 1, airport.latitude() - 1),
            Node::new(airport.longitude(), airport.latitude() - 1),
            Node::new(airport.longitude() - 1, airport.latitude() - 1),
            // Runway to the west
            // Node::new(airport.longitude() - 1, airport.latitude()),
            Node::new(airport.longitude() - 1, airport.latitude() + 1),
        ];

        neighbors
            .iter()
            .for_each(|node| assert!(!map.routing_grid().contains(node)));

        assert_eq!(MAP_WIDTH * MAP_HEIGHT - 7, map.routing_grid().len());
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
