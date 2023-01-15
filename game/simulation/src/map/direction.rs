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
    pub fn as_tuple(&self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::NorthEast => (-1, -1),
            Self::East => (-1, 0),
            Self::SouthEast => (-1, 1),
            Self::South => (0, 1),
            Self::SouthWest => (1, 1),
            Self::West => (1, 0),
            Self::NorthWest => (1, -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Direction>();
    }
}
