use bevy::prelude::*;

use crate::command::{Command, CommandBus};
use crate::components::{AirplaneId, FlightPlan};
use crate::event::{Event, EventBus};
use crate::map::Map;

pub fn update_flight_plan(
    map: Res<Map>,
    mut query: Query<(&AirplaneId, &mut FlightPlan)>,
    mut command_bus: Local<CommandBus>,
    event_bus: Local<EventBus>,
) {
    while let Ok(command) = command_bus.receiver().get_mut().try_recv() {
        let (airplane_id, new_flight_plan) = match command {
            Command::UpdateFlightPlan(airplane_id, flight_plan) => (airplane_id, flight_plan),
            _ => continue,
        };

        for (id, mut current_flight_plan) in query.iter_mut() {
            if *id == airplane_id {
                if new_flight_plan
                    .validate(&current_flight_plan, map.routing_grid())
                    .is_ok()
                {
                    *current_flight_plan = new_flight_plan.clone();

                    event_bus
                        .sender()
                        .get()
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
