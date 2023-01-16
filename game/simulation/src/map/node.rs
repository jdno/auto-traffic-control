use crate::map::Direction;
use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Node {
    pub(super) longitude: u32,
    pub(super) latitude: u32,
    pub(super) restricted: bool,
}

impl Node {
    pub fn longitude(&self) -> u32 {
        self.longitude
    }

    pub fn latitude(&self) -> u32 {
        self.latitude
    }

    pub fn is_restricted(&self) -> bool {
        self.restricted
    }

    pub fn is_neighbor(&self, potential_neighbor: &Node) -> bool {
        let delta_x = potential_neighbor.longitude() as i32 - self.longitude() as i32;
        let delta_y = potential_neighbor.latitude() as i32 - self.latitude() as i32;

        delta_x.abs() <= 1 && delta_y.abs() <= 1
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

impl From<Node> for auto_traffic_control::v1::Node {
    fn from(node: Node) -> Self {
        Self {
            longitude: node.longitude() as i32,
            latitude: node.latitude() as i32,
            restricted: node.is_restricted(),
        }
    }
}

impl From<&auto_traffic_control::v1::Node> for Node {
    fn from(node: &auto_traffic_control::v1::Node) -> Self {
        Self {
            longitude: node.longitude as u32,
            latitude: node.latitude as u32,
            restricted: node.restricted,
        }
    }
}

impl ops::Sub for &Node {
    type Output = Direction;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.longitude() as i32 - rhs.longitude() as i32;
        let y = self.latitude() as i32 - rhs.latitude() as i32;

        if x == 0 {
            if y <= 0 {
                return Direction::South;
            }
            if y > 0 {
                return Direction::North;
            }
        }
        if x < 0 {
            if y == 0 {
                return Direction::East;
            }
            if y < 0 {
                return Direction::SouthEast;
            }
            if y > 0 {
                return Direction::NorthEast;
            }
        }
        if x > 0 {
            if y == 0 {
                return Direction::West;
            }
            if y < 0 {
                return Direction::SouthWest;
            }
            if y > 0 {
                return Direction::NorthWest;
            }
        }

        panic!("failed to determine direction");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Node {
        pub fn new(longitude: u32, latitude: u32, restricted: bool) -> Self {
            Self {
                longitude,
                latitude,
                restricted,
            }
        }
    }

    #[test]
    fn is_neighbor_true() {
        let node = Node::new(0, 0, false);
        let neighbor = Node::new(1, 1, false);

        assert!(node.is_neighbor(&neighbor));
    }

    #[test]
    fn is_neighbor_false() {
        let node = Node::new(0, 0, false);
        let neighbor = Node::new(2, 0, false);

        assert!(!node.is_neighbor(&neighbor));
    }

    #[test]
    fn trait_from_api_node() {
        let node = auto_traffic_control::v1::Node {
            longitude: 0,
            latitude: 0,
            restricted: false,
        };

        assert_eq!(Node::new(0, 0, false), (&node).into());
    }

    #[test]
    fn trait_to_api_node() {
        let node = Node::new(0, 0, false);

        assert_eq!(
            auto_traffic_control::v1::Node {
                longitude: 0,
                latitude: 0,
                restricted: false,
            },
            node.into()
        );
    }

    #[test]
    fn trait_sub_north() {
        let node1 = Node {
            longitude: 0,
            latitude: 1,
            restricted: false,
        };
        let node2 = Node {
            longitude: 0,
            latitude: 0,
            restricted: false,
        };

        assert_eq!(Direction::North, &node1 - &node2);
    }

    #[test]
    fn trait_sub_north_east() {
        let node1 = Node {
            longitude: 0,
            latitude: 1,
            restricted: false,
        };
        let node2 = Node {
            longitude: 1,
            latitude: 0,
            restricted: false,
        };

        assert_eq!(Direction::NorthEast, &node1 - &node2);
    }

    #[test]
    fn trait_sub_east() {
        let node1 = Node {
            longitude: 0,
            latitude: 0,
            restricted: false,
        };
        let node2 = Node {
            longitude: 1,
            latitude: 0,
            restricted: false,
        };

        assert_eq!(Direction::East, &node1 - &node2);
    }

    #[test]
    fn trait_sub_south_east() {
        let node1 = Node {
            longitude: 0,
            latitude: 0,
            restricted: false,
        };
        let node2 = Node {
            longitude: 1,
            latitude: 1,
            restricted: false,
        };

        assert_eq!(Direction::SouthEast, &node1 - &node2);
    }

    #[test]
    fn trait_sub_south() {
        let node1 = Node {
            longitude: 0,
            latitude: 0,
            restricted: false,
        };
        let node2 = Node {
            longitude: 0,
            latitude: 1,
            restricted: false,
        };

        assert_eq!(Direction::South, &node1 - &node2);
    }

    #[test]
    fn trait_sub_south_west() {
        let node1 = Node {
            longitude: 1,
            latitude: 0,
            restricted: false,
        };
        let node2 = Node {
            longitude: 0,
            latitude: 1,
            restricted: false,
        };

        assert_eq!(Direction::SouthWest, &node1 - &node2);
    }

    #[test]
    fn trait_sub_west() {
        let node1 = Node {
            longitude: 1,
            latitude: 0,
            restricted: false,
        };
        let node2 = Node {
            longitude: 0,
            latitude: 0,
            restricted: false,
        };

        assert_eq!(Direction::West, &node1 - &node2);
    }

    #[test]
    fn trait_sub_north_west() {
        let node1 = Node {
            longitude: 1,
            latitude: 1,
            restricted: false,
        };
        let node2 = Node {
            longitude: 0,
            latitude: 0,
            restricted: false,
        };

        assert_eq!(Direction::NorthWest, &node1 - &node2);
    }

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
