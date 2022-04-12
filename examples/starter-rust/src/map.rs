use crate::{Coord, Point};
use auto_traffic_control::v1::Tag;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Airport {
    pub coord: Coord,
    pub tag: Tag,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Map {
    pub airports: Vec<Airport>,
    pub routing_grid: Vec<Coord>,
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn coord_at(&self, x: i32, y: i32) -> Coord {
        let x = x + self.width as i32 / 2;
        let y = y + self.height as i32 / 2;

        let index = (y * self.width as i32) + x;

        *self.routing_grid.get(index as usize).unwrap()
    }

    pub fn is_within_bounds(&self, point: &Point) -> bool {
        point.x.abs() <= self.width as i32 / 2 && point.y.abs() <= self.height as i32 / 2
    }
}
