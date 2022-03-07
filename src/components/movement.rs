use bevy::prelude::*;

use crate::map::Direction;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Component)]
pub struct Movement {
    speed: f32,
    direction: Direction,
}

impl Movement {
    pub fn new(speed: f32, direction: Direction) -> Self {
        Self { speed, direction }
    }

    pub fn speed(&self) -> f32 {
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
