use crate::behavior::Observable;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::bus::{Event, Sender};
use crate::component::{AirplaneId, FlightPlan, Tag};
use crate::map::{Location, Node};

#[derive(Clone, Debug)]
#[allow(dead_code)] // TODO: Remove when path finding is implemented
pub struct Airplane {
    event_bus: Sender<Event>,

    id: AirplaneId,
    location: Location,
    flight_plan: FlightPlan,
    travelled_route: Vec<Arc<Node>>,
    tag: Tag,
}

impl Airplane {
    #[allow(dead_code)] // TODO: Remove when airplanes get spawned
    pub fn new(event_bus: Sender<Event>, id: AirplaneId, tag: Tag, start: Arc<Node>) -> Self {
        let airplane = Self {
            event_bus,
            id,
            location: (&start).into(),
            flight_plan: FlightPlan::default(),
            travelled_route: vec![start],
            tag,
        };

        airplane
            .notify(Event::AirplaneDetected(
                airplane.id.clone(),
                airplane.location,
                airplane.flight_plan.clone(),
                airplane.tag,
            ))
            .expect("failed to send AirplaneDetected event");

        airplane
    }
}

impl Display for Airplane {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Airplane {}", self.id)
    }
}

impl Observable for Airplane {
    fn event_bus(&self) -> &Sender<Event> {
        &self.event_bus
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::channel;

    #[test]
    fn trait_display() {
        let (sender, _receiver) = channel(1);

        let airplane = Airplane::new(
            sender,
            AirplaneId::default(),
            Tag::Blue,
            Arc::new(Node::default()),
        );

        assert_eq!("Airplane AT-0000", airplane.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Airplane>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Airplane>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Airplane>();
    }
}
