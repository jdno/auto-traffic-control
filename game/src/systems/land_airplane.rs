use bevy::prelude::*;

use crate::components::{FlightPlan, Landing};
use crate::map::Map;

pub fn land_airplane(
    mut commands: Commands,
    map: Res<Map>,
    query: Query<(Entity, &FlightPlan), Without<Landing>>,
) {
    for (entity, flight_plan) in query.iter() {
        if flight_plan.get().len() != 1 {
            continue;
        }

        let final_node = flight_plan.get().get(0).unwrap();

        for airport in map.airports() {
            if final_node == airport.node() {
                commands.entity(entity).insert(Landing);
            }
        }
    }
}
