use std::fmt::{Display, Formatter};

use super::Coordinate;

/// The dimension of a tile
///
/// Tiles must have the same size as the textures that are used to render them. This game uses
/// textures with a size of 32 by 32 pixels, and thus tiles must be 32 pixels high and wide as well.
pub(super) const TILE_SIZE: i32 = 32;

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
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl From<&Coordinate> for Tile {
    fn from(coordinate: &Coordinate) -> Self {
        let x = coordinate.x() / TILE_SIZE;
        let y = coordinate.y() / TILE_SIZE;

        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::{Coordinate, Tile, TILE_SIZE};

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
    fn trait_from_0_coordinate() {
        let coordinate = Coordinate::new(0, 0);

        let tile = Tile::from(&coordinate);

        assert_eq!(0, tile.x);
        assert_eq!(0, tile.y);
    }

    #[test]
    fn trait_from_coordinate_smaller_than_tile_size() {
        let coordinate = Coordinate::new(TILE_SIZE / 2, TILE_SIZE / 2);

        let tile = Tile::from(&coordinate);

        assert_eq!(0, tile.x);
        assert_eq!(0, tile.y);
    }

    #[test]
    fn trait_from_coordinate_greater_than_tile_size() {
        let coordinate = Coordinate::new(TILE_SIZE * 2, TILE_SIZE * 3);

        let tile = Tile::from(&coordinate);

        assert_eq!(2, tile.x);
        assert_eq!(3, tile.y);
    }

    #[test]
    fn trait_from_negative_coordinate() {
        let coordinate = Coordinate::new(TILE_SIZE * -2, TILE_SIZE * -3);

        let tile = Tile::from(&coordinate);

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
