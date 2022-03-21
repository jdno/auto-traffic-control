use bevy::prelude::*;

use crate::map::Node;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Component)]
pub struct TravelledRoute(Vec<Node>);

impl TravelledRoute {
    pub fn new(route: Vec<Node>) -> Self {
        Self(route)
    }

    pub fn get(&self) -> &Vec<Node> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Vec<Node> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::TravelledRoute;

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
}
