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
