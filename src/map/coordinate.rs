use std::fmt::{Display, Formatter};

use crate::TILE_SIZE;

use super::Tile;

/// An arbitrary coordinate on the game's two-dimensional plane
///
/// The game is implemented with 2D graphics and a top-down camera. Players "float" above the game
/// and can observe how entities move below them. Every point on the ground can be represented by a
/// `Coordinate` with an `x` and a `y` value.
///
/// The coordinates map to [Bevy's coordinate system][bevy-coords], and thus determine where
/// entities are rendered on screen.
///
/// [bevy-coords]: https://bevy-cheatbook.github.io/features/coords.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
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

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl From<&Tile> for Coordinate {
    fn from(tile: &Tile) -> Self {
        let x = tile.x() * TILE_SIZE;
        let y = tile.y() * TILE_SIZE;

        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::{Coordinate, Tile, TILE_SIZE};

    #[test]
    fn x() {
        let coordinate = Coordinate::new(1, 2);

        assert_eq!(1, coordinate.x());
    }

    #[test]
    fn y() {
        let coordinate = Coordinate::new(1, 2);

        assert_eq!(2, coordinate.y());
    }

    #[test]
    fn trait_from_0_tile() {
        let tile = Tile::new(0, 0);

        let coordinate = Coordinate::from(&tile);

        assert_eq!(0, coordinate.x);
        assert_eq!(0, coordinate.y);
    }

    #[test]
    fn trait_from_positive_tile() {
        let tile = Tile::new(2, 3);

        let coordinate = Coordinate::from(&tile);

        assert_eq!(2 * TILE_SIZE, coordinate.x);
        assert_eq!(3 * TILE_SIZE, coordinate.y);
    }

    #[test]
    fn trait_from_negative_tile() {
        let tile = Tile::new(-2, -3);

        let coordinate = Coordinate::from(&tile);

        assert_eq!(-2 * TILE_SIZE, coordinate.x);
        assert_eq!(-3 * TILE_SIZE, coordinate.y);
    }

    #[test]
    fn trait_display() {
        let coordinate = Coordinate::new(1, 2);

        assert_eq!("Coordinate { x: 1, y: 2 }", &coordinate.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Coordinate>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Coordinate>();
    }
}
