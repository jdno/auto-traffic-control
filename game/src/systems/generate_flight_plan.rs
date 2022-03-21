use bevy::prelude::*;
use rand::prelude::*;

use crate::components::{AirplaneId, FlightPlan, TravelledRoute};
use crate::event::{Event, EventBus};
use crate::map::{Node, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};

const FLIGHT_PLAN_LENGTH: usize = 4;

pub fn generate_flight_plan(
    mut query: Query<(&AirplaneId, &mut FlightPlan, &TravelledRoute)>,
    event_bus: Local<EventBus>,
) {
    for (airplane_id, mut flight_plan, travelled_route) in query.iter_mut() {
        if !flight_plan.get().is_empty() {
            continue;
        }

        let new_flight_plan = generate_random_plan(travelled_route);
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
pub fn generate_random_plan(travelled_route: &TravelledRoute) -> FlightPlan {
    let mut flight_plan = Vec::new();

    let mut reversed_route = travelled_route.get().iter().rev();

    let mut current_node = *reversed_route
        .next()
        .expect("travelled route must include the starting location");
    let mut previous_node = reversed_route.next().cloned();
    let mut next_node;

    for _ in 0..FLIGHT_PLAN_LENGTH {
        next_node = pick_next_tile(&current_node, &previous_node);
        flight_plan.push(next_node);

        if next_node.longitude() == 0 && next_node.latitude() == 0 {
            break;
        }

        previous_node = Some(current_node);
        current_node = next_node;
    }

    FlightPlan::new(flight_plan.iter().rev().cloned().collect())
}

fn pick_next_tile(current_tile: &Node, previous_tile: &Option<Node>) -> Node {
    let mut potential_tiles = current_tile.neighbors();

    // Mustn't the previous tile
    if let Some(previous_tile) = previous_tile {
        if let Some(index) = potential_tiles
            .iter()
            .position(|tile| tile == previous_tile)
        {
            potential_tiles.remove(index);
        }
    }

    // Shouldn't be too close to the edge of the map
    let potential_tiles: Vec<Node> = potential_tiles
        .iter()
        .filter(|tile| {
            tile.longitude().abs() != *MAP_WIDTH_RANGE.end()
                && tile.latitude().abs() != *MAP_HEIGHT_RANGE.end()
        })
        .cloned()
        .collect();

    // Pick random neighbor of the current tile
    let mut rng = thread_rng();

    *potential_tiles
        .choose(&mut rng)
        .expect("failed to choose random tile")
}
