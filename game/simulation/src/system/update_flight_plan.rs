use std::sync::Arc;

use hecs::World;
use parking_lot::Mutex;

use crate::bus::{Command, Event, Receiver, Sender};
use crate::component::{AirplaneId, FlightPlan};
use crate::map::Map;
use crate::system::System;

#[derive(Debug)]
pub struct UpdateFlightPlanSystem {
    command_bus: Receiver<Command>,
    event_bus: Sender<Event>,
    map: Arc<Mutex<Map>>,
}

impl UpdateFlightPlanSystem {
    pub fn new(
        command_bus: Receiver<Command>,
        event_bus: Sender<Event>,
        map: Arc<Mutex<Map>>,
    ) -> Self {
        Self {
            command_bus,
            event_bus,
            map,
        }
    }
}

impl System for UpdateFlightPlanSystem {
    fn update(&mut self, world: &mut World, _delta: f32) {
        while let Ok(command) = self.command_bus.try_recv() {
            let (airplane_id, new_flight_plan) = match command {
                Command::UpdateFlightPlan(airplane_id, flight_plan) => (airplane_id, flight_plan),
                _ => continue,
            };

            for (_entity, (id, flight_plan)) in world.query_mut::<(&AirplaneId, &mut FlightPlan)>()
            {
                if *id != airplane_id {
                    continue;
                }

                if new_flight_plan
                    .validate(flight_plan, self.map.lock().grid())
                    .is_err()
                {
                    // The flight plan has already been validated by the API and there is only a
                    // very small chance that it becomes invalid between the validation and the
                    // update.
                    // TODO: Log the error
                    continue;
                }

                *flight_plan = new_flight_plan.clone();

                self.event_bus
                    .send(Event::FlightPlanUpdated(airplane_id, new_flight_plan))
                    .expect("failed to send FlightPlanUpdated event");

                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bus::channel;
    use crate::component::Tag;
    use crate::map::{Location, Node};

    use super::*;

    #[test]
    fn update_ok() {
        let (command_sender, command_receiver) = channel(1);
        let (event_sender, mut event_receiver) = channel(1);
        let map = Map::test();

        let mut world = World::new();
        world.spawn((
            AirplaneId::default(),
            Location::new(0.0, 0.0),
            FlightPlan::new(vec![Arc::new(Node::new(1, 0, false))]),
            Tag::Blue,
        ));

        command_sender
            .send(Command::UpdateFlightPlan(
                AirplaneId::default(),
                FlightPlan::new(vec![
                    Arc::new(Node::new(2, 0, false)),
                    Arc::new(Node::new(1, 0, false)),
                ]),
            ))
            .unwrap();

        let mut system = UpdateFlightPlanSystem::new(command_receiver, event_sender, map);
        system.update(&mut world, 0.0);

        let event = event_receiver.try_recv().unwrap();

        assert_eq!(
            Event::FlightPlanUpdated(
                AirplaneId::default(),
                FlightPlan::new(vec![
                    Arc::new(Node::new(2, 0, false)),
                    Arc::new(Node::new(1, 0, false)),
                ]),
            ),
            event
        );
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<UpdateFlightPlanSystem>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<UpdateFlightPlanSystem>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<UpdateFlightPlanSystem>();
    }
}
