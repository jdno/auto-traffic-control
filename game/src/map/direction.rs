use std::fmt::Debug;

use bevy::prelude::Vec3;
use geo::Point;

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
    pub fn between(a: &Point<f32>, b: &Point<f32>) -> Self {
        let x = a.x() - b.x();
        let y = a.y() - b.y();

        if x == 0.0 {
            if y <= 0.0 {
                return Direction::South;
            }
            if y > 0.0 {
                return Direction::North;
            }
        }
        if x < 0.0 {
            if y == 0.0 {
                return Direction::West;
            }
            if y < 0.0 {
                return Direction::SouthWest;
            }
            if y > 0.0 {
                return Direction::NorthWest;
            }
        }
        if x > 0.0 {
            if y == 0.0 {
                return Direction::East;
            }
            if y < 0.0 {
                return Direction::SouthEast;
            }
            if y > 0.0 {
                return Direction::NorthEast;
            }
        }

        panic!("failed to determine direction");
    }

    pub fn to_degree(self) -> f32 {
        match self {
            Direction::North => 90.0,
            Direction::NorthEast => 45.0,
            Direction::East => 0.0,
            Direction::SouthEast => 315.0,
            Direction::South => 270.0,
            Direction::SouthWest => 225.0,
            Direction::West => 180.0,
            Direction::NorthWest => 135.0,
        }
    }

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
