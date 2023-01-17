use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::component::Tag;
use crate::map::{Airport, Grid, Map, Node};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Maps {
    Sandbox,
    #[cfg(test)]
    Test,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MapLoader;

impl MapLoader {
    pub fn load(map: Maps) -> Map {
        match map {
            Maps::Sandbox => Self::parse(map.to_string(), include_str!("../../maps/sandbox.txt")),
            #[cfg(test)]
            Maps::Test => Self::parse(map.to_string(), include_str!("../../maps/test.txt")),
        }
    }

    fn parse(name: String, map: &str) -> Map {
        let mut nodes = Vec::new();
        let mut airports = Vec::new();
        let mut spawns = Vec::new();

        let mut width = 0;
        let height = map.lines().count();

        for (y, line) in map.lines().enumerate() {
            let line_width = line.len();

            if width == 0 {
                width = line_width;
            } else if width != line_width {
                panic!("map width is not consistent");
            }

            for (x, tile) in line.chars().enumerate() {
                let node = match tile {
                    '#' | 'S' => Node {
                        longitude: x as u32,
                        latitude: y as u32,
                        restricted: true,
                    },
                    _ => Node {
                        longitude: x as u32,
                        latitude: y as u32,
                        restricted: false,
                    },
                };
                let node = Arc::new(node);

                if tile == 'B' {
                    let airport = Airport::new(node.clone(), Tag::Blue);
                    airports.push(airport);
                }
                if tile == 'R' {
                    let airport = Airport::new(node.clone(), Tag::Red);
                    airports.push(airport);
                }
                if tile == 'S' {
                    spawns.push(node.clone());
                }

                nodes.push(node);
            }
        }

        if width == 0 && height == 0 {
            panic!("map is empty");
        }

        if airports.is_empty() {
            panic!("map has no airports");
        }

        if spawns.is_empty() {
            panic!("map has no spawns");
        }

        Map {
            name,

            width: width as u32,
            height: height as u32,

            airports,
            grid: Grid::new(width as u32, height as u32, nodes),
            spawns,
        }
    }
}

impl Display for MapLoader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MapLoader")
    }
}

impl Display for Maps {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Maps::Sandbox => "Sandbox",
            #[cfg(test)]
            Maps::Test => "Test",
        };

        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox() {
        MapLoader::load(Maps::Sandbox);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<MapLoader>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<MapLoader>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<MapLoader>();
    }
}
