use std::fmt::{Display, Formatter};

use geo::{point, Point};

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

    pub fn to_point(self) -> Point<i32> {
        point!(x: self.x * TILE_SIZE, y: self.y * TILE_SIZE)
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

#[cfg(test)]
mod tests {
    use geo::{point, Point};

    use super::{Tile, TILE_SIZE};

    #[test]
    fn x() {
        let tile = Tile::new(1, 2);

        assert_eq!(1, tile.x());
    }

    #[test]
    fn y() {
        let tile = Tile::new(1, 2);

        assert_eq!(2, tile.y());
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
