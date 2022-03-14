use bevy::prelude::*;

use crate::command::CommandBus;
use crate::components::{AirplaneId, FlightPlan};
use crate::Command;

pub fn update_flight_plan(
    mut query: Query<(&AirplaneId, &mut FlightPlan)>,
    mut command_bus: Local<CommandBus>,
) {
    while let Ok(command) = command_bus.receiver().try_recv() {
        let Command::UpdateFlightPlan(airplane_id, new_flight_plan) = command;

        for (id, mut current_flight_plan) in query.iter_mut() {
            if *id == airplane_id {
                *current_flight_plan = new_flight_plan;
                break;
            }
        }
    }
}
