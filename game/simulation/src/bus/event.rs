use std::fmt::{Display, Formatter};

use crate::component::{AirplaneId, FlightPlan, Tag};
use crate::map::{Location, Map};

#[derive(Clone, Debug)]
pub enum Event {
    AirplaneDetected(AirplaneId, Location, FlightPlan, Tag),
    GameStarted(Map),
    GameStopped,
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Event>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Event>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Event>();
    }
}
