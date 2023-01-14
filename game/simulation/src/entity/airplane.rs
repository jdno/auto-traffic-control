use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::behavior::{Observable, Updateable};
use crate::bus::{Command, Event, Receiver, Sender};
use crate::component::{AirplaneId, FlightPlan, Tag};
use crate::map::{Location, Node};

#[derive(Debug)]
#[allow(dead_code)] // TODO: Remove when path finding is implemented
pub struct Airplane {
    command_bus: Receiver<Command>,
    event_bus: Sender<Event>,

    id: AirplaneId,
    location: Location,
    flight_plan: FlightPlan,
    travelled_route: Vec<Arc<Node>>,
    tag: Tag,
}

impl Airplane {
    pub fn new(
        command_bus: Receiver<Command>,
        event_bus: Sender<Event>,
        id: AirplaneId,
        tag: Tag,
        start_node: Arc<Node>,
        first_node: Arc<Node>,
    ) -> Self {
        let airplane = Self {
            command_bus,
            event_bus,

            id,
            location: (&start_node).into(),
            flight_plan: FlightPlan::new(vec![first_node]),
            travelled_route: vec![start_node],
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

impl Updateable for Airplane {
    fn update(&mut self, _delta: f32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::bus::channel;

    use super::*;

    #[test]
    fn trait_display() {
        let (_command_sender, command_receiver) = channel(1);
        let (event_sender, _event_receiver) = channel(1);

        let airplane = Airplane::new(
            command_receiver,
            event_sender,
            AirplaneId::default(),
            Tag::Blue,
            Arc::new(Node::default()),
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
