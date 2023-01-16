use auto_traffic_control::v1::{Airplane, Map};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::component::{AirplaneId, FlightPlan, Tag};
use crate::map::{Airport, Grid, Location, Node};

#[derive(Clone, PartialEq, Debug)]
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
        let string = match self {
            Event::AirplaneCollided(_, _) => "AirplaneCollided",
            Event::AirplaneDetected(_, _, _, _) => "AirplaneDetected",
            Event::AirplaneLanded(_) => "AirplaneLanded",
            Event::AirplaneMoved(_, _) => "AirplaneMoved",
            Event::FlightPlanUpdated(_, _) => "FlightPlanUpdated",
            Event::GameStarted(_, _, _, _) => "GameStarted",
            Event::GameStopped(_) => "GameStopped",
            Event::LandingAborted(_) => "LandingAborted",
        };

        write!(f, "{}", string)
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
    fn trait_display() {
        let event = Event::GameStopped(0);
        assert_eq!("GameStopped", event.to_string());
    }

    #[test]
    fn trait_from_airplane_collided() {
        let event = Event::AirplaneCollided(
            AirplaneId::new("AT-0001".into()),
            AirplaneId::new("AT-0002".into()),
        );

        let expected = auto_traffic_control::v1::stream_response::Event::AirplaneCollided(
            auto_traffic_control::v1::AirplaneCollided {
                id1: "AT-0001".into(),
                id2: "AT-0002".into(),
            },
        );

        assert_eq!(expected, event.into());
    }

    #[test]
    fn trait_from_airplane_detected() {
        let event = Event::AirplaneDetected(
            AirplaneId::new("AT-0001".into()),
            Location::new(1.0, 2.0),
            FlightPlan::new(vec![Arc::new(Node::new(1, 1, false))]),
            Tag::Blue,
        );

        let expected = auto_traffic_control::v1::stream_response::Event::AirplaneDetected(
            auto_traffic_control::v1::AirplaneDetected {
                airplane: Some(Airplane {
                    id: "AT-0001".into(),
                    point: Some(auto_traffic_control::v1::Point { x: 1, y: 2 }),
                    flight_plan: vec![auto_traffic_control::v1::Node {
                        longitude: 1,
                        latitude: 1,
                        restricted: false,
                    }],
                    tag: auto_traffic_control::v1::Tag::Blue.into(),
                }),
            },
        );

        assert_eq!(expected, event.into());
    }

    #[test]
    fn trait_from_airplane_landed() {
        let event = Event::AirplaneLanded(AirplaneId::new("AT-0001".into()));

        let expected = auto_traffic_control::v1::stream_response::Event::AirplaneLanded(
            auto_traffic_control::v1::AirplaneLanded {
                id: "AT-0001".into(),
            },
        );

        assert_eq!(expected, event.into());
    }

    #[test]
    fn trait_from_airplane_moved() {
        let event =
            Event::AirplaneMoved(AirplaneId::new("AT-0001".into()), Location::new(1.0, 2.0));

        let expected = auto_traffic_control::v1::stream_response::Event::AirplaneMoved(
            auto_traffic_control::v1::AirplaneMoved {
                id: "AT-0001".into(),
                point: Some(auto_traffic_control::v1::Point { x: 1, y: 2 }),
            },
        );

        assert_eq!(expected, event.into());
    }

    #[test]
    fn trait_from_flight_plan_updated() {
        let event = Event::FlightPlanUpdated(
            AirplaneId::new("AT-0001".into()),
            FlightPlan::new(vec![Arc::new(Node::new(1, 1, false))]),
        );

        let expected = auto_traffic_control::v1::stream_response::Event::FlightPlanUpdated(
            auto_traffic_control::v1::FlightPlanUpdated {
                id: "AT-0001".into(),
                flight_plan: vec![auto_traffic_control::v1::Node {
                    longitude: 1,
                    latitude: 1,
                    restricted: false,
                }],
            },
        );

        assert_eq!(expected, event.into());
    }

    #[test]
    fn trait_from_game_started() {
        let node = Arc::new(Node::new(0, 0, false));

        let event = Event::GameStarted(
            vec![Airport::new(node.clone(), Tag::Blue)],
            Grid::new(1, 1, vec![node]),
            1,
            1,
        );

        let expected = auto_traffic_control::v1::stream_response::Event::GameStarted(
            auto_traffic_control::v1::GameStarted {
                map: Some(Map {
                    airports: vec![auto_traffic_control::v1::Airport {
                        node: Some(auto_traffic_control::v1::Node {
                            longitude: 0,
                            latitude: 0,
                            restricted: false,
                        }),
                        tag: auto_traffic_control::v1::Tag::Blue.into(),
                    }],
                    routing_grid: vec![auto_traffic_control::v1::Node {
                        longitude: 0,
                        latitude: 0,
                        restricted: false,
                    }],
                    width: 1,
                    height: 1,
                }),
            },
        );

        assert_eq!(expected, event.into());
    }

    #[test]
    fn trait_from_game_stopped() {
        let event = Event::GameStopped(100);

        let expected = auto_traffic_control::v1::stream_response::Event::GameStopped(
            auto_traffic_control::v1::GameStopped { score: 100 },
        );

        assert_eq!(expected, event.into());
    }

    #[test]
    fn trait_from_landing_aborted() {
        let event = Event::LandingAborted(AirplaneId::new("AT-0001".into()));

        let expected = auto_traffic_control::v1::stream_response::Event::LandingAborted(
            auto_traffic_control::v1::LandingAborted {
                id: "AT-0001".into(),
            },
        );

        assert_eq!(expected, event.into());
    }

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
