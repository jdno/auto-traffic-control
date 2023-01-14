use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

use geo::Point;

use crate::map::Node;
use crate::TILE_SIZE;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Location(Point);

impl Location {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Point::new(x, y))
    }

    pub fn x(&self) -> f64 {
        self.0.x()
    }

    pub fn y(&self) -> f64 {
        self.0.y()
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location {{ x: {}, y: {} }}", self.x(), self.y())
    }
}

impl From<&Node> for Location {
    fn from(node: &Node) -> Self {
        let x = node.longitude() * TILE_SIZE;
        let y = node.latitude() * TILE_SIZE;

        Self::new(x as f64, y as f64)
    }
}

impl From<&Arc<Node>> for Location {
    fn from(node: &Arc<Node>) -> Self {
        node.deref().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_from_node_zero() {
        let node = Node {
            longitude: 0,
            latitude: 0,
            restricted: false,
        };

        let location = Location::from(&node);

        assert_eq!(location, Location::new(0.0, 0.0));
    }

    #[test]
    fn trait_from_node_nonzero() {
        let node = Node {
            longitude: 1,
            latitude: 2,
            restricted: false,
        };

        let location = Location::from(&node);

        assert_eq!(location, Location::new(64.0, 128.0));
    }

    #[test]
    fn trait_display() {
        let location = Location::new(1.0, 2.0);
        assert_eq!("Location { x: 1, y: 2 }", location.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Location>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Location>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Location>();
    }
}
