use bevy::prelude::*;

use atc::v1::Node as ApiNode;

use crate::api::IntoApi;
use crate::map::Tile;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Component)]
pub struct FlightPlan(Vec<Tile>);

impl FlightPlan {
    pub fn new(flight_plan: Vec<Tile>) -> Self {
        Self(flight_plan)
    }

    pub fn get(&self) -> &Vec<Tile> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.0
    }
}

impl IntoApi for FlightPlan {
    type ApiType = Vec<ApiNode>;

    fn into_api(self) -> Self::ApiType {
        self.0.iter().map(|node| node.into_api()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::FlightPlan;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<FlightPlan>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<FlightPlan>();
    }
}
