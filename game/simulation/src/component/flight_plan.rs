use std::fmt::{Display, Formatter};
use std::ops::Deref;
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
            self.has_restricted_nodes(grid),
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

    fn has_restricted_nodes(&self, grid: &Grid<Arc<Node>>) -> Result<(), FlightPlanError> {
        for node in self.0.iter() {
            if let Some(node) = grid.get(node.longitude(), node.latitude()) {
                if node.is_restricted() {
                    return Err(FlightPlanError::RestrictedNode);
                }
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

impl From<FlightPlan> for Vec<auto_traffic_control::v1::Node> {
    fn from(flight_plan: FlightPlan) -> Self {
        flight_plan
            .get()
            .iter()
            .rev()
            .map(|node| (*node.deref()).into())
            .collect()
    }
}

impl From<FlightPlanError> for auto_traffic_control::v1::update_flight_plan_error::ValidationError {
    fn from(error: FlightPlanError) -> Self {
        match error {
            FlightPlanError::NodeOutsideMap => Self::NodeOutsideMap,
            FlightPlanError::InvalidStep => Self::InvalidStep,
            FlightPlanError::SharpTurn => Self::SharpTurn,
            FlightPlanError::InvalidStart => Self::InvalidStart,
            FlightPlanError::RestrictedNode => Self::RestrictedNode,
        }
    }
}

impl From<Vec<auto_traffic_control::v1::Node>> for FlightPlan {
    fn from(flight_plan: Vec<auto_traffic_control::v1::Node>) -> Self {
        Self::new(
            flight_plan
                .iter()
                .rev()
                .map(|node| Arc::new(node.into()))
                .collect(),
        )
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
    fn validate_ok() {
        let grid = Grid::new(1, 1, vec![Arc::new(Node::new(0, 0, false))]);
        let flight_plan = FlightPlan::new(vec![Arc::new(Node::new(0, 0, false))]);

        assert!(flight_plan.validate(&flight_plan, &grid).is_ok());
    }

    #[test]
    fn validate_error() {
        let grid = Grid::new(
            2,
            2,
            vec![
                Arc::new(Node::new(0, 0, false)),
                Arc::new(Node::new(1, 0, false)),
                Arc::new(Node::new(0, 1, false)),
                Arc::new(Node::new(1, 1, true)),
            ],
        );

        let previous_flight_plan = FlightPlan::new(vec![Arc::new(Node::new(0, 0, false))]);
        let flight_plan = FlightPlan::new(vec![
            Arc::new(Node::new(1, 1, false)),
            Arc::new(Node::new(3, 1, false)),
            Arc::new(Node::new(1, 1, false)),
        ]);

        let errors = flight_plan
            .validate(&previous_flight_plan, &grid)
            .unwrap_err();

        assert_eq!(
            vec![
                FlightPlanError::NodeOutsideMap,
                FlightPlanError::InvalidStep,
                FlightPlanError::InvalidStart,
                FlightPlanError::SharpTurn,
                FlightPlanError::RestrictedNode,
            ],
            errors
        );
    }

    #[test]
    fn is_within_map_bounds_ok() {
        let grid = Grid::new(1, 1, vec![Arc::new(Node::new(0, 0, false))]);
        let flight_plan = FlightPlan::new(vec![Arc::new(Node::new(0, 0, false))]);

        assert!(flight_plan.is_within_map_bounds(&grid).is_ok());
    }

    #[test]
    fn is_within_map_bounds_error() {
        let grid = Grid::new(1, 1, vec![Arc::new(Node::new(0, 0, false))]);
        let flight_plan = FlightPlan::new(vec![Arc::new(Node::new(1, 1, false))]);

        let error = flight_plan.is_within_map_bounds(&grid).unwrap_err();

        assert_eq!(FlightPlanError::NodeOutsideMap, error);
    }

    #[test]
    fn is_in_logical_order_ok() {
        let flight_plan = FlightPlan::new(vec![
            Arc::new(Node::new(0, 0, false)),
            Arc::new(Node::new(1, 0, false)),
        ]);

        assert!(flight_plan.is_in_logical_order().is_ok());
    }

    #[test]
    fn is_in_logical_order_error() {
        let flight_plan = FlightPlan::new(vec![
            Arc::new(Node::new(0, 0, false)),
            Arc::new(Node::new(2, 0, false)),
        ]);

        let error = flight_plan.is_in_logical_order().unwrap_err();

        assert_eq!(FlightPlanError::InvalidStep, error);
    }

    #[test]
    fn has_invalid_first_node_ok() {
        let node = Arc::new(Node::new(0, 0, false));

        let previous_flight_plan = FlightPlan::new(vec![node.clone()]);
        let flight_plan = FlightPlan::new(vec![node]);

        assert!(flight_plan
            .has_invalid_first_node(&previous_flight_plan)
            .is_ok());
    }

    #[test]
    fn has_invalid_first_node_error() {
        let previous_flight_plan = FlightPlan::new(vec![Arc::new(Node::new(0, 0, false))]);
        let flight_plan = FlightPlan::new(vec![Arc::new(Node::new(1, 0, false))]);

        let error = flight_plan
            .has_invalid_first_node(&previous_flight_plan)
            .unwrap_err();

        assert_eq!(FlightPlanError::InvalidStart, error);
    }

    #[test]
    fn has_sharp_turns_ok() {
        let flight_plan = FlightPlan::new(vec![
            Arc::new(Node::new(0, 0, false)),
            Arc::new(Node::new(1, 0, false)),
            Arc::new(Node::new(2, 0, false)),
        ]);

        assert!(flight_plan.has_sharp_turns().is_ok());
    }

    #[test]
    fn has_sharp_turns_err() {
        let flight_plan = FlightPlan::new(vec![
            Arc::new(Node::new(0, 0, false)),
            Arc::new(Node::new(1, 0, false)),
            Arc::new(Node::new(0, 0, false)),
        ]);

        let error = flight_plan.has_sharp_turns().unwrap_err();

        assert_eq!(FlightPlanError::SharpTurn, error);
    }

    #[test]
    fn has_restricted_nodes_ok() {
        let grid = Grid::new(1, 1, vec![Arc::new(Node::new(0, 0, false))]);
        let flight_plan = FlightPlan::new(vec![Arc::new(Node::new(0, 0, false))]);

        assert!(flight_plan.has_restricted_nodes(&grid).is_ok());
    }

    #[test]
    fn has_restricted_nodes_error() {
        let grid = Grid::new(1, 1, vec![Arc::new(Node::new(0, 0, true))]);
        let flight_plan = FlightPlan::new(vec![Arc::new(Node::new(0, 0, true))]);

        let error = flight_plan.has_restricted_nodes(&grid).unwrap_err();

        assert_eq!(FlightPlanError::RestrictedNode, error);
    }

    #[test]
    fn trait_from_error() {
        let error = FlightPlanError::NodeOutsideMap;

        let error: auto_traffic_control::v1::update_flight_plan_error::ValidationError =
            error.into();

        assert_eq!(
            auto_traffic_control::v1::update_flight_plan_error::ValidationError::NodeOutsideMap,
            error
        );
    }

    #[test]
    fn trait_from_node() {
        let flight_plan = FlightPlan::from(vec![
            auto_traffic_control::v1::Node {
                longitude: 0,
                latitude: 0,
                restricted: false,
            },
            auto_traffic_control::v1::Node {
                longitude: 1,
                latitude: 0,
                restricted: false,
            },
        ]);

        assert_eq!(
            FlightPlan::new(vec![
                Arc::new(Node::new(1, 0, false)),
                Arc::new(Node::new(0, 0, false)),
            ]),
            flight_plan
        );
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
