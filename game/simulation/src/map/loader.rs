use std::fmt::{Display, Formatter};

use crate::map::{Map, Node};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Maps {
    Sandbox,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MapLoader;

impl MapLoader {
    pub fn load(map: Maps) -> Map {
        match map {
            Maps::Sandbox => Self::parse(map.to_string(), include_str!("../../maps/sandbox.txt")),
        }
    }

    fn parse(name: String, map: &str) -> Map {
        let mut nodes = Vec::new();

        let mut width = 0;
        let height = map.lines().count();

        for (x, line) in map.lines().enumerate() {
            let line_width = line.len();

            if width == 0 {
                width = line_width;
            } else if width != line_width {
                panic!("map width is not consistent");
            }

            for (y, tile) in line.chars().enumerate() {
                let node = match tile {
                    '#' => Node::restricted(x as u32, y as u32),
                    _ => Node::unrestricted(x as u32, y as u32),
                };

                nodes.push(node);
            }
        }

        if width == 0 && height == 0 {
            panic!("map is empty");
        }

        Map {
            name,
            width: width as u32,
            height: height as u32,
            grid: nodes,
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
