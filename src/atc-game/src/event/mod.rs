use atc::v1::stream_response::Event as ApiEvent;
use atc::v1::{
    Airplane, AirplaneDetected, AirplaneLanded, AirplaneMoved, FlightPlanUpdated, GameStarted,
    GameStopped,
};

use crate::api::IntoApi;
use crate::components::{AirplaneId, FlightPlan, Location};

pub use self::bus::{EventBus, EventReceiver, EventSender};

mod bus;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Event {
    AirplaneDetected(AirplaneId, Location, FlightPlan),
    AirplaneLanded(AirplaneId),
    AirplaneMoved(AirplaneId, Location),
    FlightPlanUpdated(AirplaneId, FlightPlan),
    GameStarted,
    GameStopped,
}

impl IntoApi for Event {
    type ApiType = atc::v1::stream_response::Event;

    fn into_api(self) -> Self::ApiType {
        match self {
            Event::AirplaneDetected(id, location, flight_plan) => {
                ApiEvent::AirplaneDetected(AirplaneDetected {
                    airplane: Some(Airplane {
                        id: id.into_api(),
                        location: Some(location.into_api()),
                        flight_plan: flight_plan.into_api(),
                    }),
                })
            }
            Event::AirplaneLanded(id) => {
                ApiEvent::AirplaneLanded(AirplaneLanded { id: id.into_api() })
            }
            Event::AirplaneMoved(id, location) => ApiEvent::AirplaneMoved(AirplaneMoved {
                id: id.into_api(),
                location: Some(location.into_api()),
            }),
            Event::FlightPlanUpdated(id, flight_plan) => {
                ApiEvent::FlightPlanUpdated(FlightPlanUpdated {
                    id: id.into_api(),
                    flight_plan: flight_plan.into_api(),
                })
            }
            Event::GameStarted => ApiEvent::GameStarted(GameStarted {}),
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
