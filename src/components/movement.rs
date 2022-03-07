use bevy::prelude::*;

use crate::map::Direction;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Component)]
pub struct Movement {
    speed: usize,
    direction: Direction,
}

impl Movement {
    pub fn new(speed: usize, direction: Direction) -> Self {
        Self { speed, direction }
    }

    pub fn speed(&self) -> usize {
        self.speed
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::Movement;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Movement>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Movement>();
    }
}
