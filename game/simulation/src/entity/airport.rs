use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::component::Tag;
use crate::map::Node;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Airport {
    node: Arc<Node>,
    tag: Tag,
}

impl Airport {
    pub fn new(node: Arc<Node>, tag: Tag) -> Self {
        Self { node, tag }
    }
}

impl Display for Airport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Airport")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
