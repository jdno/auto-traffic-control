use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::component::Tag;
use crate::map::{Location, Node};

#[derive(Clone, PartialEq, Debug)]
pub struct Airport {
    node: Arc<Node>,
    location: Location,
    tag: Tag,
}

impl Airport {
    pub fn new(node: Arc<Node>, tag: Tag) -> Self {
        let location = (&node).into();

        Self {
            node,
            location,
            tag,
        }
    }

    pub fn node(&self) -> &Arc<Node> {
        &self.node
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn tag(&self) -> Tag {
        self.tag
    }
}

impl Display for Airport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Airport")
    }
}

impl From<Airport> for auto_traffic_control::v1::Airport {
    fn from(airport: Airport) -> Self {
        let tag: auto_traffic_control::v1::Tag = airport.tag.into();

        Self {
            node: Some((*airport.node).into()),
            tag: tag.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let airport = Airport::new(Arc::new(Node::new(0, 0, false)), Tag::Blue);
        assert_eq!("Airport", airport.to_string());
    }

    #[test]
    fn trait_from_airport() {
        let airport = Airport::new(Arc::new(Node::new(0, 0, false)), Tag::Blue);

        assert_eq!(
            auto_traffic_control::v1::Airport {
                node: Some(auto_traffic_control::v1::Node {
                    longitude: 0,
                    latitude: 0,
                    restricted: false,
                }),
                tag: auto_traffic_control::v1::Tag::Blue.into(),
            },
            airport.into()
        );
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Airport>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Airport>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Airport>();
    }
}
