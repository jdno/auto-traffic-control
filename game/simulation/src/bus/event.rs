use auto_traffic_control::v1::{Airplane, Map};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::component::{AirplaneId, FlightPlan, Tag};
use crate::map::{Airport, Grid, Location, Node};

#[derive(Clone, Debug)]
pub enum Event {
    AirplaneCollided(AirplaneId, AirplaneId),
    AirplaneDetected(AirplaneId, Location, FlightPlan, Tag),
    AirplaneLanded(AirplaneId),
    AirplaneMoved(AirplaneId, Location),
    FlightPlanUpdated(AirplaneId, FlightPlan),
    GameStarted(Vec<Airport>, Grid<Arc<Node>>, u32, u32),
    GameStopped(u32),
    LandingAborted(AirplaneId),
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event")
    }
}

impl From<Event> for auto_traffic_control::v1::stream_response::Event {
    fn from(event: Event) -> Self {
        match event {
            Event::AirplaneCollided(id1, id2) => {
                Self::AirplaneCollided(auto_traffic_control::v1::AirplaneCollided {
                    id1: id1.into(),
                    id2: id2.into(),
                })
            }
            Event::AirplaneDetected(id, location, flight_plan, tag) => {
                let tag: auto_traffic_control::v1::Tag = tag.into();

                Self::AirplaneDetected(auto_traffic_control::v1::AirplaneDetected {
                    airplane: Some(Airplane {
                        id: id.to_string(),
                        point: Some(location.into()),
                        flight_plan: flight_plan.into(),
                        tag: tag.into(),
                    }),
                })
            }
            Event::AirplaneLanded(id) => {
                Self::AirplaneLanded(auto_traffic_control::v1::AirplaneLanded {
                    id: id.to_string(),
                })
            }
            Event::AirplaneMoved(id, location) => {
                Self::AirplaneMoved(auto_traffic_control::v1::AirplaneMoved {
                    id: id.to_string(),
                    point: Some(location.into()),
                })
            }
            Event::FlightPlanUpdated(id, flight_plan) => {
                Self::FlightPlanUpdated(auto_traffic_control::v1::FlightPlanUpdated {
                    id: id.to_string(),
                    flight_plan: flight_plan.into(),
                })
            }
            Event::GameStarted(airports, grid, width, height) => {
                Self::GameStarted(auto_traffic_control::v1::GameStarted {
                    map: Some(Map {
                        airports: airports.into_iter().map(|airport| airport.into()).collect(),
                        routing_grid: grid
                            .elements()
                            .iter()
                            .map(|node| (*node.clone()).into())
                            .collect(),
                        width,
                        height,
                    }),
                })
            }
            Event::GameStopped(score) => {
                Self::GameStopped(auto_traffic_control::v1::GameStopped { score })
            }
            Event::LandingAborted(id) => {
                Self::LandingAborted(auto_traffic_control::v1::LandingAborted {
                    id: id.to_string(),
                })
            }
        }
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
