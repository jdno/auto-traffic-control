use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::map::Node;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FlightPlan(Vec<Arc<Node>>);

impl FlightPlan {
    pub fn new(flight_plan: Vec<Arc<Node>>) -> Self {
        Self(flight_plan)
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
