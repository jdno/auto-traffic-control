use bevy::prelude::*;

use crate::command::{Command, CommandBus};
use crate::components::{AirplaneId, FlightPlan};
use crate::event::{Event, EventBus};

pub fn update_flight_plan(
    mut query: Query<(&AirplaneId, &mut FlightPlan)>,
    mut command_bus: Local<CommandBus>,
    event_bus: Local<EventBus>,
) {
    while let Ok(command) = command_bus.receiver().try_recv() {
        let Command::UpdateFlightPlan(airplane_id, new_flight_plan) = command;

        for (id, mut current_flight_plan) in query.iter_mut() {
            if *id == airplane_id {
                if new_flight_plan.validate(&current_flight_plan).is_ok() {
                    *current_flight_plan = new_flight_plan.clone();

                    event_bus
                        .sender()
                        .send(Event::FlightPlanUpdated(
                            airplane_id.clone(),
                            new_flight_plan,
                        ))
                        .expect("failed to send event"); // TODO: Handle error

                    break;
                } else {
                    // TODO: Handle error
                }
            }
        }
    }
}
