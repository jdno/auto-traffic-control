use hecs::World;

use crate::bus::{Command, Event, Receiver, Sender};
use crate::component::{AirplaneId, FlightPlan};
use crate::system::System;

#[derive(Debug)]
pub struct UpdateFlightPlanSystem {
    command_bus: Receiver<Command>,
    event_bus: Sender<Event>,
}

impl UpdateFlightPlanSystem {
    pub fn new(command_bus: Receiver<Command>, event_bus: Sender<Event>) -> Self {
        Self {
            command_bus,
            event_bus,
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
    use super::*;

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
