use bevy::prelude::*;

use crate::map::Tile;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Component)]
pub struct TravelledRoute(Vec<Tile>);

impl TravelledRoute {
    pub fn new(route: Vec<Tile>) -> Self {
        Self(route)
    }

    pub fn get(&self) -> &Vec<Tile> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Vec<Tile> {
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
