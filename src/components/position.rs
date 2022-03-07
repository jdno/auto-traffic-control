use bevy::prelude::*;

use crate::map::Coordinate;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Component)]
pub struct Position(Coordinate);

impl Position {
    pub fn new(coordinate: Coordinate) -> Self {
        Self(coordinate)
    }

    pub fn get(&self) -> Coordinate {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Position;
    use crate::map::Coordinate;

    #[test]
    fn get() {
        let position = Position::new(Coordinate::default());

        assert_eq!(Coordinate::new(0, 0), position.get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Position>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Position>();
    }
}
