use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::map::{Grid, Node};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum FlightPlanError {
    NodeOutsideMap,
    InvalidStep,
    SharpTurn,
    InvalidStart,
    RestrictedNode,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FlightPlan(Vec<Arc<Node>>);

impl FlightPlan {
    pub fn new(flight_plan: Vec<Arc<Node>>) -> Self {
        Self(flight_plan)
    }

    pub fn get(&self) -> &Vec<Arc<Node>> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Vec<Arc<Node>> {
        &mut self.0
    }

    pub fn validate(
        &self,
        previous_flight_plan: &FlightPlan,
        grid: &Grid<Arc<Node>>,
    ) -> Result<(), Vec<FlightPlanError>> {
        let errors: Vec<FlightPlanError> = vec![
            self.is_within_map_bounds(grid),
            self.is_in_logical_order(),
            self.has_invalid_first_node(previous_flight_plan),
            self.has_sharp_turns(),
            self.has_restricted_nodes(),
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

    fn is_within_map_bounds(&self, grid: &Grid<Arc<Node>>) -> Result<(), FlightPlanError> {
        for node in self.0.iter() {
            if node.longitude() < grid.width() && node.latitude() < grid.height() {
                continue;
            }

            return Err(FlightPlanError::NodeOutsideMap);
        }

        Ok(())
    }

    fn is_in_logical_order(&self) -> Result<(), FlightPlanError> {
        for window in self.0.windows(2) {
            let previous = &window[0];
            let next = &window[1];

            if !previous.is_neighbor(next) {
                return Err(FlightPlanError::InvalidStep);
            }
        }

        Ok(())
    }

    fn has_invalid_first_node(
        &self,
        previous_flight_plan: &FlightPlan,
    ) -> Result<(), FlightPlanError> {
        if self.0.last() == previous_flight_plan.get().last() {
            Ok(())
        } else {
            Err(FlightPlanError::InvalidStart)
        }
    }

    fn has_sharp_turns(&self) -> Result<(), FlightPlanError> {
        for window in self.0.windows(3) {
            let previous = &window[0];
            let next = &window[2];

            if previous == next {
                return Err(FlightPlanError::SharpTurn);
            }
        }

        Ok(())
    }

    fn has_restricted_nodes(&self) -> Result<(), FlightPlanError> {
        for node in self.0.iter() {
            if node.is_restricted() {
                return Err(FlightPlanError::RestrictedNode);
            }
        }

        Ok(())
    }
}

impl Display for FlightPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlightPlan")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let flight_plan = FlightPlan::default();
        assert_eq!("FlightPlan", flight_plan.to_string());
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

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<FlightPlan>();
    }
}
