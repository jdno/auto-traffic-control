use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Node {
    longitude: u32,
    latitude: u32,
    restricted: bool,
}

impl Node {
    pub fn restricted(longitude: u32, latitude: u32) -> Self {
        Self {
            longitude,
            latitude,
            restricted: true,
        }
    }

    pub fn unrestricted(longitude: u32, latitude: u32) -> Self {
        Self {
            longitude,
            latitude,
            restricted: false,
        }
    }

    pub fn longitude(&self) -> u32 {
        self.longitude
    }

    pub fn latitude(&self) -> u32 {
        self.latitude
    }

    #[allow(dead_code)] // TODO: Remove when path finding is implemented
    pub fn is_restricted(&self) -> bool {
        self.restricted
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let restriction = match self.restricted {
            true => "restricted",
            false => "unrestricted",
        };
        write!(
            f,
            "{} node {{ x: {}, y: {} }}",
            restriction, self.longitude, self.latitude
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let node = Node {
            longitude: 1,
            latitude: 2,
            restricted: true,
        };

        assert_eq!(format!("{}", node), "restricted node { x: 1, y: 2 }");
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Node>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Node>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Node>();
    }
}
