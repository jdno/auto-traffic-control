use bevy::prelude::*;

use crate::components::{AirplaneId, FlightPlan};
use crate::{Event, EventBus};

pub fn despawn_airplane(
    mut commands: Commands,
    query: Query<(Entity, &AirplaneId, &FlightPlan)>,
    event_bus: Local<EventBus>,
) {
    for (entity, airplane_id, flight_plan) in query.iter() {
        if flight_plan.get().is_empty() {
            commands.entity(entity).despawn();

            event_bus
                .sender()
                .send(Event::AirplaneLanded(airplane_id.clone()))
                .expect("failed to send event"); // TODO: Handle error
        }
    }
}
