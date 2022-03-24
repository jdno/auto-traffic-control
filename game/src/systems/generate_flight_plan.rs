use bevy::prelude::*;
use rand::prelude::*;

use crate::components::{AirplaneId, FlightPlan, Landing, TravelledRoute};
use crate::event::{Event, EventBus};
use crate::map::{Map, Node, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};

const FLIGHT_PLAN_LENGTH: usize = 4;

pub fn generate_flight_plan(
    map: Res<Map>,
    mut query: Query<(&AirplaneId, &mut FlightPlan, &TravelledRoute), Without<Landing>>,
    event_bus: Local<EventBus>,
) {
    for (airplane_id, mut flight_plan, travelled_route) in query.iter_mut() {
        if !flight_plan.get().is_empty() {
            continue;
        }

        let new_flight_plan = generate_random_plan(travelled_route, &map);
        *flight_plan = new_flight_plan.clone();

        event_bus
            .sender()
            .send(Event::FlightPlanUpdated(
                airplane_id.clone(),
                new_flight_plan,
            ))
            .expect("failed to send event"); // TODO: Handle error
    }
}

// TODO: Refactor into a shared module
pub fn generate_random_plan(travelled_route: &TravelledRoute, map: &Res<Map>) -> FlightPlan {
    let mut flight_plan = Vec::new();

    let mut reversed_route = travelled_route.get().iter().rev();

    let mut current_node = *reversed_route
        .next()
        .expect("travelled route must include the starting location");
    let mut previous_node = reversed_route.next().cloned();
    let mut next_node;

    for _ in 0..FLIGHT_PLAN_LENGTH {
        next_node = pick_next_tile(&current_node, &previous_node, map);
        flight_plan.push(next_node);

        previous_node = Some(current_node);
        current_node = next_node;
    }

    FlightPlan::new(flight_plan.iter().rev().cloned().collect())
}

fn pick_next_tile(current_tile: &Node, previous_tile: &Option<Node>, map: &Map) -> Node {
    let airports: Vec<&Node> = map
        .airports()
        .iter()
        .map(|airport| airport.node())
        .collect();

    let potential_tiles: Vec<Node> = current_tile
        .neighbors()
        .into_iter()
        .filter(|node| &Some(*node) != previous_tile)
        .filter(|node| !airports.contains(&node))
        .filter(|node| !node.is_restricted())
        .filter(|tile| {
            // Shouldn't be too close to the edge of the map
            tile.longitude().abs() != *MAP_WIDTH_RANGE.end()
                && tile.latitude().abs() != *MAP_HEIGHT_RANGE.end()
        })
        .collect();

    // Pick random neighbor of the current tile
    let mut rng = thread_rng();

    *potential_tiles
        .choose(&mut rng)
        .expect("failed to choose random tile")
}
