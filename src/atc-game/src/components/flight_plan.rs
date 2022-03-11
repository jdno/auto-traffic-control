use bevy::prelude::*;

use atc::v1::Node as ApiNode;

use crate::api::IntoApi;
use crate::map::{Tile, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ValidationError {
    HasSharpTurns,
    NodeOutOfBounds,
    NotInLogicalOrder,
}

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
        self.0.iter().rev().map(|node| node.into_api()).collect()
    }
}

fn validate(flight_plan: &[Tile]) -> Result<(), Vec<ValidationError>> {
    let errors: Vec<ValidationError> = vec![
        is_within_map_bounds(flight_plan),
        is_in_logical_order(flight_plan),
        has_sharp_turns(flight_plan),
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

fn is_within_map_bounds(flight_plan: &[Tile]) -> Result<(), ValidationError> {
    for node in flight_plan.iter() {
        if !MAP_WIDTH_RANGE.contains(&node.x()) {
            return Err(ValidationError::NodeOutOfBounds);
        }
        if !MAP_HEIGHT_RANGE.contains(&node.y()) {
            return Err(ValidationError::NodeOutOfBounds);
        }
    }

    Ok(())
}

fn is_in_logical_order(flight_plan: &[Tile]) -> Result<(), ValidationError> {
    for window in flight_plan.windows(2) {
        let previous = window[0];
        let next = window[1];

        if !previous.is_neighbor(&next) {
            return Err(ValidationError::NotInLogicalOrder);
        }
    }

    Ok(())
}

fn has_sharp_turns(flight_plan: &[Tile]) -> Result<(), ValidationError> {
    for window in flight_plan.windows(3) {
        let previous = window[0];
        let next = window[2];

        if previous == next {
            return Err(ValidationError::HasSharpTurns);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::components::flight_plan::{
        has_sharp_turns, is_in_logical_order, is_within_map_bounds, validate,
    };
    use crate::components::ValidationError;
    use crate::map::{Tile, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};

    use super::FlightPlan;

    #[test]
    fn validate_with_valid_plan() {
        let flight_plan = vec![Tile::new(0, 0), Tile::new(1, 0), Tile::new(2, 0)];

        let result = validate(&flight_plan);

        assert!(result.is_ok());
    }

    #[test]
    fn validate_with_invalid_plan() {
        let x = MAP_WIDTH_RANGE.start() - 1;
        let y = MAP_HEIGHT_RANGE.start() - 1;

        let flight_plan = vec![Tile::new(x, y), Tile::new(0, 0)];

        let result = validate(&flight_plan);

        assert_eq!(
            vec![
                ValidationError::NodeOutOfBounds,
                ValidationError::NotInLogicalOrder
            ],
            result.err().unwrap()
        );
    }

    #[test]
    fn is_within_map_bounds_with_valid_plan() {
        let flight_plan = vec![Tile::new(0, 0), Tile::new(1, 0), Tile::new(2, 0)];

        let result = is_within_map_bounds(&flight_plan);

        assert!(result.is_ok());
    }

    #[test]
    fn is_within_map_bounds_with_invalid_plan() {
        let x = MAP_WIDTH_RANGE.start() - 1;
        let y = MAP_HEIGHT_RANGE.start() - 1;

        let flight_plan = vec![Tile::new(x, y)];

        let result = is_within_map_bounds(&flight_plan);

        assert!(result.is_err());
    }

    #[test]
    fn is_in_logical_order_with_valid_plan() {
        let flight_plan = vec![Tile::new(0, 0), Tile::new(1, 0), Tile::new(2, 0)];

        let result = is_in_logical_order(&flight_plan);

        assert!(result.is_ok());
    }

    #[test]
    fn is_in_logical_order_with_invalid_plan() {
        let flight_plan = vec![Tile::new(0, 0), Tile::new(3, 3)];

        let result = is_in_logical_order(&flight_plan);

        assert!(result.is_err());
    }

    #[test]
    fn has_sharp_turns_without_turns() {
        let flight_plan = vec![Tile::new(0, 0), Tile::new(1, 0), Tile::new(1, 1)];

        let result = has_sharp_turns(&flight_plan);

        assert!(result.is_ok());
    }

    #[test]
    fn has_sharp_turns_with_turns() {
        let flight_plan = vec![Tile::new(0, 0), Tile::new(1, 0), Tile::new(0, 0)];

        let result = has_sharp_turns(&flight_plan);

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
