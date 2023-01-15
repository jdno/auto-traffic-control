use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::map::Node;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TravelledRoute(Vec<Arc<Node>>);

impl TravelledRoute {
    pub fn new(travelled_route: Vec<Arc<Node>>) -> Self {
        Self(travelled_route)
    }

    pub fn get(&self) -> &Vec<Arc<Node>> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Vec<Arc<Node>> {
        &mut self.0
    }
}

impl Display for TravelledRoute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TravelledRoute")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let travelled_route = TravelledRoute::default();
        assert_eq!("TravelledRoute", travelled_route.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<TravelledRoute>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<TravelledRoute>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<TravelledRoute>();
    }
}
