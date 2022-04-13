use std::ops::Deref;
use std::rc::Rc;

use crate::map::Map;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Coord(pub i32, pub i32, pub bool);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub map: Rc<Option<Map>>,
}

impl Point {
    pub fn distance(&self, other: &Point) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }

    pub fn neighbors(&self) -> Vec<(Point, u32)> {
        let potential_neighbors = vec![
            (
                Point {
                    x: self.x,
                    y: self.y + 1,
                    map: self.map.clone(),
                },
                1,
            ),
            (
                Point {
                    x: self.x + 1,
                    y: self.y + 1,
                    map: self.map.clone(),
                },
                2,
            ),
            (
                Point {
                    x: self.x + 1,
                    y: self.y,
                    map: self.map.clone(),
                },
                1,
            ),
            (
                Point {
                    x: self.x + 1,
                    y: self.y - 1,
                    map: self.map.clone(),
                },
                2,
            ),
            (
                Point {
                    x: self.x,
                    y: self.y - 1,
                    map: self.map.clone(),
                },
                1,
            ),
            (
                Point {
                    x: self.x - 1,
                    y: self.y - 1,
                    map: self.map.clone(),
                },
                2,
            ),
            (
                Point {
                    x: self.x - 1,
                    y: self.y,
                    map: self.map.clone(),
                },
                1,
            ),
            (
                Point {
                    x: self.x - 1,
                    y: self.y + 1,
                    map: self.map.clone(),
                },
                2,
            ),
        ];

        let map: &Map = self.map.deref().as_ref().unwrap();

        potential_neighbors
            .into_iter()
            .filter(|(point, _)| map.is_within_bounds(point))
            .filter(|(point, _)| !map.coord_at(point.x, point.y).2)
            .collect()
    }
}
