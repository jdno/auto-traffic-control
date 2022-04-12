use auto_traffic_control::v1::stream_response::Event as ApiEvent;
use auto_traffic_control::v1::{
    Airplane, AirplaneCollided, AirplaneDetected, AirplaneLanded, AirplaneMoved, FlightPlanUpdated,
    GameStarted, GameStopped, LandingAborted,
};

use crate::api::AsApi;
use crate::components::{AirplaneId, FlightPlan, Location, Tag};
use crate::map::Map;
use crate::resources::Score;

pub use self::bus::{EventBus, EventReceiver, EventSender};

mod bus;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Event {
    AirplaneCollided(AirplaneId, AirplaneId),
    AirplaneDetected(AirplaneId, Location, FlightPlan, Tag),
    AirplaneLanded(AirplaneId),
    AirplaneMoved(AirplaneId, Location),
    FlightPlanUpdated(AirplaneId, FlightPlan),
    LandingAborted(AirplaneId),
    GameStarted(Map),
    GameStopped(Score),
}

impl AsApi for Event {
    type ApiType = auto_traffic_control::v1::stream_response::Event;

    fn as_api(&self) -> Self::ApiType {
        match self {
            Event::AirplaneCollided(airplane_id1, airplane_id2) => {
                ApiEvent::AirplaneCollided(AirplaneCollided {
                    id1: airplane_id1.as_api(),
                    id2: airplane_id2.as_api(),
                })
            }
            Event::AirplaneDetected(id, location, flight_plan, tag) => {
                ApiEvent::AirplaneDetected(AirplaneDetected {
                    airplane: Some(Airplane {
                        id: id.as_api(),
                        point: Some(location.as_api()),
                        flight_plan: flight_plan.as_api(),
                        tag: tag.as_api().into(),
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
            Event::LandingAborted(id) => {
                ApiEvent::LandingAborted(LandingAborted { id: id.as_api() })
            }
            Event::GameStarted(map) => ApiEvent::GameStarted(GameStarted {
                map: Some(map.as_api()),
            }),
            Event::GameStopped(score) => ApiEvent::GameStopped(GameStopped { score: score.get() }),
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
