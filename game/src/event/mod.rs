use atc::v1::stream_response::Event as ApiEvent;
use atc::v1::{
    Airplane, AirplaneCollided, AirplaneDetected, AirplaneLanded, AirplaneMoved, FlightPlanUpdated,
    GameStarted, GameStopped,
};

use crate::api::AsApi;
use crate::components::{AirplaneId, FlightPlan, Location};
use crate::map::Map;

pub use self::bus::{EventBus, EventReceiver, EventSender};

mod bus;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Event {
    AirplaneCollided(AirplaneId, AirplaneId),
    AirplaneDetected(AirplaneId, Location, FlightPlan),
    AirplaneLanded(AirplaneId),
    AirplaneMoved(AirplaneId, Location),
    FlightPlanUpdated(AirplaneId, FlightPlan),
    GameStarted(Map),
    GameStopped,
}

impl AsApi for Event {
    type ApiType = atc::v1::stream_response::Event;

    fn as_api(&self) -> Self::ApiType {
        match self {
            Event::AirplaneCollided(airplane_id1, airplane_id2) => {
                ApiEvent::AirplaneCollided(AirplaneCollided {
                    id1: airplane_id1.as_api(),
                    id2: airplane_id2.as_api(),
                })
            }
            Event::AirplaneDetected(id, location, flight_plan) => {
                ApiEvent::AirplaneDetected(AirplaneDetected {
                    airplane: Some(Airplane {
                        id: id.as_api(),
                        point: Some(location.as_api()),
                        flight_plan: flight_plan.as_api(),
                    }),
                })
            }
            Event::AirplaneLanded(id) => {
                ApiEvent::AirplaneLanded(AirplaneLanded { id: id.as_api() })
            }
            Event::AirplaneMoved(id, location) => ApiEvent::AirplaneMoved(AirplaneMoved {
                id: id.as_api(),
                point: Some(location.as_api()),
            }),
            Event::FlightPlanUpdated(id, flight_plan) => {
                ApiEvent::FlightPlanUpdated(FlightPlanUpdated {
                    id: id.as_api(),
                    flight_plan: flight_plan.as_api(),
                })
            }
            Event::GameStarted(map) => ApiEvent::GameStarted(GameStarted {
                map: Some(map.as_api()),
            }),
            Event::GameStopped => ApiEvent::GameStopped(GameStopped {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Event;

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
}
