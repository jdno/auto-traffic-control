use bevy::prelude::*;

use atc::v1::update_flight_plan_error::ValidationError;
use atc::v1::Node as ApiNode;

use crate::api::IntoApi;
use crate::map::{Tile, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};

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

    pub fn validate(&self, previous_flight_plan: &FlightPlan) -> Result<(), Vec<ValidationError>> {
        let errors: Vec<ValidationError> = vec![
            self.is_within_map_bounds(),
            self.is_in_logical_order(),
            self.has_invalid_first_node(previous_flight_plan),
            self.has_sharp_turns(),
        ]
        .iter()
        .filter_map(|result| result.err())
        .collect();

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn is_within_map_bounds(&self) -> Result<(), ValidationError> {
        for node in self.0.iter() {
            if !MAP_WIDTH_RANGE.contains(&node.x()) {
                return Err(ValidationError::NodeOutOfBounds);
            }
            if !MAP_HEIGHT_RANGE.contains(&node.y()) {
                return Err(ValidationError::NodeOutOfBounds);
            }
        }

        Ok(())
    }

    fn is_in_logical_order(&self) -> Result<(), ValidationError> {
        for window in self.0.windows(2) {
            let previous = window[0];
            let next = window[1];

            if !previous.is_neighbor(&next) {
                return Err(ValidationError::NotInLogicalOrder);
            }
        }

        Ok(())
    }

    fn has_invalid_first_node(
        &self,
        previous_flight_plan: &FlightPlan,
    ) -> Result<(), ValidationError> {
        if self.0.last() == previous_flight_plan.get().last() {
            Ok(())
        } else {
            Err(ValidationError::InvalidFirstNode)
        }
    }

    fn has_sharp_turns(&self) -> Result<(), ValidationError> {
        for window in self.0.windows(3) {
            let previous = window[0];
            let next = window[2];

            if previous == next {
                return Err(ValidationError::HasSharpTurns);
            }
        }

        Ok(())
    }
}

impl From<&Vec<atc::v1::Node>> for FlightPlan {
    fn from(api_flight_plan: &Vec<atc::v1::Node>) -> Self {
        let tiles = api_flight_plan
            .iter()
            .rev()
            .map(|node| Tile::new(node.longitude, node.latitude))
            .collect();

        FlightPlan(tiles)
    }
}

impl IntoApi for FlightPlan {
    type ApiType = Vec<ApiNode>;

    fn into_api(self) -> Self::ApiType {
        self.0.iter().rev().map(|node| node.into_api()).collect()
    }
}

#[cfg(test)]
mod tests {
    use atc::v1::update_flight_plan_error::ValidationError;

    use crate::map::{Tile, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};

    use super::FlightPlan;

    #[test]
    fn validate_with_valid_plan() {
        let previous_flight_plan =
            FlightPlan(vec![Tile::new(2, 0), Tile::new(1, 0), Tile::new(0, 0)]);
        let new_flight_plan = FlightPlan(vec![Tile::new(1, 1), Tile::new(1, 0), Tile::new(0, 0)]);

        let result = new_flight_plan.validate(&previous_flight_plan);

        assert!(result.is_ok());
    }

    #[test]
    fn validate_with_invalid_plan() {
        let x = *MAP_WIDTH_RANGE.start();
        let y = *MAP_HEIGHT_RANGE.start();

        let previous_flight_plan = FlightPlan(vec![Tile::new(0, 0), Tile::new(x, y)]);
        let new_flight_plan = FlightPlan(vec![Tile::new(x - 1, y - 1), Tile::new(0, 0)]);

        let result = new_flight_plan.validate(&previous_flight_plan);

        assert_eq!(
            vec![
                ValidationError::NodeOutOfBounds,
                ValidationError::NotInLogicalOrder,
                ValidationError::InvalidFirstNode
            ],
            result.err().unwrap()
        );
    }

    #[test]
    fn is_within_map_bounds_with_valid_plan() {
        let flight_plan = FlightPlan(vec![Tile::new(0, 0), Tile::new(1, 0), Tile::new(2, 0)]);

        let result = flight_plan.is_within_map_bounds();

        assert!(result.is_ok());
    }

    #[test]
    fn is_within_map_bounds_with_invalid_plan() {
        let x = MAP_WIDTH_RANGE.start() - 1;
        let y = MAP_HEIGHT_RANGE.start() - 1;

        let flight_plan = FlightPlan(vec![Tile::new(x, y)]);

        let result = flight_plan.is_within_map_bounds();

        assert!(result.is_err());
    }

    #[test]
    fn is_in_logical_order_with_valid_plan() {
        let flight_plan = FlightPlan(vec![Tile::new(0, 0), Tile::new(1, 0), Tile::new(2, 0)]);

        let result = flight_plan.is_in_logical_order();

        assert!(result.is_ok());
    }

    #[test]
    fn is_in_logical_order_with_invalid_plan() {
        let flight_plan = FlightPlan(vec![Tile::new(0, 0), Tile::new(3, 3)]);

        let result = flight_plan.is_in_logical_order();

        assert!(result.is_err());
    }

    #[test]
    fn has_invalid_first_node_with_valid_plan() {
        let previous_flight_plan = FlightPlan(vec![Tile::new(1, 0), Tile::new(0, 0)]);
        let new_flight_plan = FlightPlan(vec![Tile::new(0, 1), Tile::new(0, 0)]);

        let result = new_flight_plan.has_invalid_first_node(&previous_flight_plan);

        assert!(result.is_ok());
    }

    #[test]
    fn has_invalid_first_node_with_invalid_plan() {
        let previous_flight_plan = FlightPlan(vec![Tile::new(0, 0), Tile::new(1, 0)]);
        let new_flight_plan = FlightPlan(vec![Tile::new(1, 0), Tile::new(0, 0)]);

        let result = new_flight_plan.has_invalid_first_node(&previous_flight_plan);

        assert!(result.is_err());
    }

    #[test]
    fn has_sharp_turns_without_turns() {
        let flight_plan = FlightPlan(vec![Tile::new(0, 0), Tile::new(1, 0), Tile::new(1, 1)]);

        let result = flight_plan.has_sharp_turns();

        assert!(result.is_ok());
    }

    #[test]
    fn has_sharp_turns_with_turns() {
        let flight_plan = FlightPlan(vec![Tile::new(0, 0), Tile::new(1, 0), Tile::new(0, 0)]);

        let result = flight_plan.has_sharp_turns();

        assert!(result.is_err());
    }

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
