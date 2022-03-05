use std::fmt::{Display, Formatter};

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
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate {{ x:{}, y: {} }}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::Coordinate;

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
