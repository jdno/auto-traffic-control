use bevy::prelude::*;

use auto_traffic_control::v1::update_flight_plan_error::ValidationError;
use auto_traffic_control::v1::Node as ApiNode;

use crate::api::AsApi;
use crate::map::{Node, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Component)]
pub struct FlightPlan(Vec<Node>);

impl FlightPlan {
    pub fn new(flight_plan: Vec<Node>) -> Self {
        Self(flight_plan)
    }

    pub fn next(&self) -> Option<&Node> {
        self.0.last()
    }

    pub fn get(&self) -> &Vec<Node> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Vec<Node> {
        &mut self.0
    }

    pub fn validate(
        &self,
        previous_flight_plan: &FlightPlan,
        routing_grid: &[Node],
    ) -> Result<(), Vec<ValidationError>> {
        let errors: Vec<ValidationError> = vec![
            self.is_within_map_bounds(),
            self.is_in_logical_order(),
            self.has_invalid_first_node(previous_flight_plan),
            self.has_sharp_turns(),
            self.has_restricted_nodes(routing_grid),
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
            if !MAP_WIDTH_RANGE.contains(&node.longitude()) {
                return Err(ValidationError::NodeOutsideMap);
            }
            if !MAP_HEIGHT_RANGE.contains(&node.latitude()) {
                return Err(ValidationError::NodeOutsideMap);
            }
        }

        Ok(())
    }

    fn is_in_logical_order(&self) -> Result<(), ValidationError> {
        for window in self.0.windows(2) {
            let previous = window[0];
            let next = window[1];

            if !previous.is_neighbor(&next) {
                return Err(ValidationError::InvalidStep);
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
            Err(ValidationError::InvalidStart)
        }
    }

    fn has_sharp_turns(&self) -> Result<(), ValidationError> {
        for window in self.0.windows(3) {
            let previous = window[0];
            let next = window[2];

            if previous == next {
                return Err(ValidationError::SharpTurn);
            }
        }

        Ok(())
    }

    fn has_restricted_nodes(&self, routing_grid: &[Node]) -> Result<(), ValidationError> {
        for node in self.0.iter() {
            if !routing_grid.contains(node) {
                return Err(ValidationError::RestrictedNode);
            }
        }

        Ok(())
    }
}

impl From<&Vec<auto_traffic_control::v1::Node>> for FlightPlan {
    fn from(api_flight_plan: &Vec<auto_traffic_control::v1::Node>) -> Self {
        let tiles = api_flight_plan
            .iter()
            .rev()
            .map(|node| Node::unrestricted(node.longitude, node.latitude))
            .collect();

        FlightPlan(tiles)
    }
}

impl AsApi for FlightPlan {
    type ApiType = Vec<ApiNode>;

    fn as_api(&self) -> Self::ApiType {
        self.0.iter().rev().map(|node| node.as_api()).collect()
    }
}

#[cfg(test)]
mod tests {
    use auto_traffic_control::v1::update_flight_plan_error::ValidationError;

    use crate::map::{Node, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};

    use super::FlightPlan;

    fn routing_grid() -> Vec<Node> {
        let mut nodes = Vec::new();

        for y in -3..=3 {
            for x in -3..=3 {
                nodes.push(Node::unrestricted(x, y));
            }
        }

        nodes
    }

    #[test]
    fn validate_with_valid_plan() {
        let previous_flight_plan = FlightPlan(vec![
            Node::unrestricted(2, 0),
            Node::unrestricted(1, 0),
            Node::unrestricted(0, 0),
        ]);
        let new_flight_plan = FlightPlan(vec![
            Node::unrestricted(1, 1),
            Node::unrestricted(1, 0),
            Node::unrestricted(0, 0),
        ]);

        let result = new_flight_plan.validate(&previous_flight_plan, &routing_grid());

        assert!(result.is_ok());
    }

    #[test]
    fn validate_with_invalid_plan() {
        let x = *MAP_WIDTH_RANGE.start();
        let y = *MAP_HEIGHT_RANGE.start();

        let previous_flight_plan =
            FlightPlan(vec![Node::unrestricted(0, 0), Node::unrestricted(x, y)]);
        let new_flight_plan = FlightPlan(vec![
            Node::unrestricted(x - 1, y - 1),
            Node::unrestricted(0, 0),
        ]);

        let result = new_flight_plan.validate(&previous_flight_plan, &routing_grid());

        assert_eq!(
            vec![
                ValidationError::NodeOutsideMap,
                ValidationError::InvalidStep,
                ValidationError::InvalidStart,
                ValidationError::RestrictedNode,
            ],
            result.err().unwrap()
        );
    }

    #[test]
    fn is_within_map_bounds_with_valid_plan() {
        let flight_plan = FlightPlan(vec![
            Node::unrestricted(0, 0),
            Node::unrestricted(1, 0),
            Node::unrestricted(2, 0),
        ]);

        let result = flight_plan.is_within_map_bounds();

        assert!(result.is_ok());
    }

    #[test]
    fn is_within_map_bounds_with_invalid_plan() {
        let x = MAP_WIDTH_RANGE.start() - 1;
        let y = MAP_HEIGHT_RANGE.start() - 1;

        let flight_plan = FlightPlan(vec![Node::unrestricted(x, y)]);

        let result = flight_plan.is_within_map_bounds();

        assert!(result.is_err());
    }

    #[test]
    fn is_in_logical_order_with_valid_plan() {
        let flight_plan = FlightPlan(vec![
            Node::unrestricted(0, 0),
            Node::unrestricted(1, 0),
            Node::unrestricted(2, 0),
        ]);

        let result = flight_plan.is_in_logical_order();

        assert!(result.is_ok());
    }

    #[test]
    fn is_in_logical_order_with_invalid_plan() {
        let flight_plan = FlightPlan(vec![Node::unrestricted(0, 0), Node::unrestricted(3, 3)]);

        let result = flight_plan.is_in_logical_order();

        assert!(result.is_err());
    }

    #[test]
    fn has_invalid_first_node_with_valid_plan() {
        let previous_flight_plan =
            FlightPlan(vec![Node::unrestricted(1, 0), Node::unrestricted(0, 0)]);
        let new_flight_plan = FlightPlan(vec![Node::unrestricted(0, 1), Node::unrestricted(0, 0)]);

        let result = new_flight_plan.has_invalid_first_node(&previous_flight_plan);

        assert!(result.is_ok());
    }

    #[test]
    fn has_invalid_first_node_with_invalid_plan() {
        let previous_flight_plan =
            FlightPlan(vec![Node::unrestricted(0, 0), Node::unrestricted(1, 0)]);
        let new_flight_plan = FlightPlan(vec![Node::unrestricted(1, 0), Node::unrestricted(0, 0)]);

        let result = new_flight_plan.has_invalid_first_node(&previous_flight_plan);

        assert!(result.is_err());
    }

    #[test]
    fn has_sharp_turns_without_turns() {
        let flight_plan = FlightPlan(vec![
            Node::unrestricted(0, 0),
            Node::unrestricted(1, 0),
            Node::unrestricted(1, 1),
        ]);

        let result = flight_plan.has_sharp_turns();

        assert!(result.is_ok());
    }

    #[test]
    fn has_sharp_turns_with_turns() {
        let flight_plan = FlightPlan(vec![
            Node::unrestricted(0, 0),
            Node::unrestricted(1, 0),
            Node::unrestricted(0, 0),
        ]);

        let result = flight_plan.has_sharp_turns();

        assert!(result.is_err());
    }

    #[test]
    fn has_restricted_nodes_without_restricted_nodes() {
        let flight_plan = FlightPlan(vec![Node::unrestricted(0, 0)]);
        let routing_grid = vec![Node::unrestricted(0, 0)];

        let result = flight_plan.has_restricted_nodes(&routing_grid);

        assert!(result.is_ok());
    }

    #[test]
    fn has_restricted_nodes_with_restricted_nodes() {
        let flight_plan = FlightPlan(vec![Node::unrestricted(1, 1)]);
        let routing_grid = vec![Node::unrestricted(0, 0)];

        let result = flight_plan.has_restricted_nodes(&routing_grid);

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
