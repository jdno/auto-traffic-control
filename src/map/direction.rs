use bevy::prelude::Vec3;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn to_vec3(self) -> Vec3 {
        match self {
            Direction::North => Vec3::new(0.0, 1.0, 0.0),
            Direction::NorthEast => Vec3::new(1.0, 1.0, 0.0),
            Direction::East => Vec3::new(1.0, 0.0, 0.0),
            Direction::SouthEast => Vec3::new(1.0, -1.0, 0.0),
            Direction::South => Vec3::new(0.0, -1.0, 0.0),
            Direction::SouthWest => Vec3::new(-1.0, -1.0, 0.0),
            Direction::West => Vec3::new(-1.0, 0.0, 0.0),
            Direction::NorthWest => Vec3::new(-1.0, 1.0, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Direction>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Direction>();
    }
}
