use atc::v1::Airport as ApiAirport;

use crate::api::AsApi;
use crate::components::Tag;
use crate::map::{Direction, Node};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Airport {
    node: Node,
    runway: Direction,
    tag: Tag,
}

impl Airport {
    pub fn new(node: Node, runway: Direction, tag: Tag) -> Self {
        Self { node, runway, tag }
    }

    pub fn node(&self) -> &Node {
        &self.node
    }

    pub fn runway(&self) -> Direction {
        self.runway
    }

    #[allow(dead_code)] // TODO: Remove when tags are introduced to flight plans
    pub fn tag(&self) -> Tag {
        self.tag
    }
}

impl AsApi for Airport {
    type ApiType = ApiAirport;

    fn as_api(&self) -> Self::ApiType {
        ApiAirport {
            node: Some(self.node.as_api()),
            tag: self.tag.as_api().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Airport;

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
}
