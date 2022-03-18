use bevy::prelude::*;
use rand::prelude::*;

use crate::components::{AirplaneId, FlightPlan, TravelledRoute};
use crate::event::{Event, EventBus};
use crate::map::{Tile, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};

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

    let mut current_tile = *reversed_route
        .next()
        .expect("travelled route must include the starting location");
    let mut previous_tile = reversed_route.next().cloned();
    let mut next_tile;

    for _ in 0..FLIGHT_PLAN_LENGTH {
        next_tile = pick_next_tile(&current_tile, &previous_tile);
        flight_plan.push(next_tile);

        if next_tile.x() == 0 && next_tile.y() == 0 {
            break;
        }

        previous_tile = Some(current_tile);
        current_tile = next_tile;
    }

    FlightPlan::new(flight_plan.iter().rev().cloned().collect())
}

fn pick_next_tile(current_tile: &Tile, previous_tile: &Option<Tile>) -> Tile {
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
    let potential_tiles: Vec<Tile> = potential_tiles
        .iter()
        .filter(|tile| {
            tile.x().abs() != *MAP_WIDTH_RANGE.end() && tile.y().abs() != *MAP_HEIGHT_RANGE.end()
        })
        .cloned()
        .collect();

    // Pick random neighbor of the current tile
    let mut rng = thread_rng();

    *potential_tiles
        .choose(&mut rng)
        .expect("failed to choose random tile")
}
