use geo::Point;

use atc::v1::stream_response::Event as ApiEvent;
use atc::v1::{
    Airplane, AirplaneDetected, AirplaneFlightPlanUpdated, AirplaneLanded, AirplaneMoved,
    Point as ApiPoint,
};

use crate::api::IntoApi;
use crate::components::{AirplaneId, FlightPlan};

pub use self::bus::{EventBus, EventSender};

mod bus;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Event {
    AirplaneDetected(AirplaneId, Point<i32>, FlightPlan),
    AirplaneLanded(AirplaneId),
    AirplaneMoved(AirplaneId, Point<i32>),
    FlightPlanUpdated(AirplaneId, FlightPlan),
}

impl IntoApi for Event {
    type ApiType = atc::v1::stream_response::Event;

    fn into_api(self) -> Self::ApiType {
        match self {
            Event::AirplaneDetected(id, point, flight_plan) => {
                ApiEvent::AirplaneDetected(AirplaneDetected {
                    airplane: Some(Airplane {
                        id: id.into_api(),
                        point: Some(ApiPoint {
                            x: point.x() as i32,
                            y: point.y() as i32,
                        }),
                        flight_plan: flight_plan.into_api(),
                    }),
                })
            }
            Event::AirplaneLanded(id) => {
                ApiEvent::AirplaneLanded(AirplaneLanded { id: id.into_api() })
            }
            Event::AirplaneMoved(id, point) => ApiEvent::AirplaneMoved(AirplaneMoved {
                id: id.into_api(),
                point: Some(ApiPoint {
                    x: point.x() as i32,
                    y: point.y() as i32,
                }),
            }),
            Event::FlightPlanUpdated(id, flight_plan) => {
                ApiEvent::AirplaneFlightPlanUpdated(AirplaneFlightPlanUpdated {
                    id: id.into_api(),
                    flight_plan: flight_plan.into_api(),
                })
            }
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
