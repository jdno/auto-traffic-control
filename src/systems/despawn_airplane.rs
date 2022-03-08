use bevy::prelude::*;

use crate::components::FlightPlan;

pub fn despawn_airplane(mut commands: Commands, query: Query<(Entity, &FlightPlan)>) {
    for (entity, flight_plan) in query.iter() {
        if flight_plan.get().is_empty() {
            commands.entity(entity).despawn();
        }
    }
}
