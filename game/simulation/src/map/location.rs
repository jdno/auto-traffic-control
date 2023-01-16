use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

use geo::{EuclideanDistance, LineInterpolatePoint, LineString, Point};

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

    pub fn euclidean_distance(&self, other: &Self) -> f64 {
        self.0.euclidean_distance(&other.0)
    }

    pub fn move_towards(&self, other: &Self, distance: f64) -> Option<Self> {
        let distance_to_other = self.euclidean_distance(other);

        if distance_to_other < distance {
            return None;
        }

        let line: LineString = vec![self.0, other.0].into();
        let fraction = distance / distance_to_other;

        Some(line.line_interpolate_point(fraction)?.into())
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location {{ x: {}, y: {} }}", self.x(), self.y())
    }
}

impl From<&auto_traffic_control::v1::Node> for Location {
    fn from(node: &auto_traffic_control::v1::Node) -> Self {
        let x = node.longitude as u32 * TILE_SIZE;
        let y = node.latitude as u32 * TILE_SIZE;

        Self::new(x as f64, y as f64)
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

impl From<Point> for Location {
    fn from(point: Point) -> Self {
        Location(point)
    }
}

impl From<Location> for auto_traffic_control::v1::Point {
    fn from(location: Location) -> Self {
        Self {
            x: location.x() as i32,
            y: location.y() as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::ApproxEq;

    use super::*;

    #[test]
    fn euclidean_distance_horizontal() {
        let location1 = Location::new(0.0, 0.0);
        let location2 = Location::new(64.0, 0.0);

        assert_eq!(64.0, location1.euclidean_distance(&location2));
    }

    #[test]
    fn euclidean_distance_vertical() {
        let location1 = Location::new(0.0, 64.0);
        let location2 = Location::new(0.0, 0.0);

        assert_eq!(64.0, location1.euclidean_distance(&location2));
    }

    #[test]
    fn euclidean_distance_diagonal() {
        let location1 = Location::new(0.0, 0.0);
        let location2 = Location::new(64.0, 64.0);

        let distance = location1.euclidean_distance(&location2);

        assert!(distance.approx_eq(90.5, (0.1, 2)));
    }

    #[test]
    fn move_towards_horizontal() {
        let location1 = Location::new(0.0, 0.0);
        let location2 = Location::new(64.0, 0.0);

        let location3 = location1.move_towards(&location2, 32.0).unwrap();

        assert_eq!(Location::new(32.0, 0.0), location3);
    }

    #[test]
    fn move_towards_vertical() {
        let location1 = Location::new(0.0, 0.0);
        let location2 = Location::new(0.0, 64.0);

        let location3 = location1.move_towards(&location2, 32.0).unwrap();

        assert_eq!(Location::new(0.0, 32.0), location3);
    }

    #[test]
    fn move_towards_diagonal() {
        let location1 = Location::new(0.0, 0.0);
        let location2 = Location::new(64.0, 64.0);

        let location3 = location1.move_towards(&location2, 32.0).unwrap();

        assert_eq!(
            Location::new(22.62741699796952, 22.62741699796952),
            location3
        );
    }

    #[test]
    fn trait_from_api_node_zero() {
        let node = auto_traffic_control::v1::Node {
            longitude: 0,
            latitude: 0,
            restricted: false,
        };

        let location = Location::from(&node);

        assert_eq!(location, Location::new(0.0, 0.0));
    }

    #[test]
    fn trait_from_api_node_nonzero() {
        let node = auto_traffic_control::v1::Node {
            longitude: 1,
            latitude: 2,
            restricted: false,
        };

        let location = Location::from(&node);

        assert_eq!(location, Location::new(64.0, 128.0));
    }

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
    fn trait_from_api_point() {
        let location = Location::new(64.0, 128.0);

        assert_eq!(
            auto_traffic_control::v1::Point { x: 64, y: 128 },
            location.into()
        );
    }

    #[test]
    fn trait_from_point() {
        let point = Point::new(64.0, 128.0);
        assert_eq!(Location::new(64.0, 128.0), point.into());
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
