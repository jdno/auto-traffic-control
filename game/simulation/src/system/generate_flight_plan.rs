use std::sync::Arc;

use parking_lot::Mutex;
use rand::prelude::*;

use crate::bus::{Event, Sender};
use crate::component::{AirplaneId, FlightPlan, TravelledRoute};
use crate::map::{Map, Node};
use crate::system::System;

const FLIGHT_PLAN_LENGTH: u32 = 4;

#[derive(Clone, Debug)]
pub struct GenerateFlightPlanSystem {
    event_bus: Sender<Event>,
    rng: ThreadRng,
    map: Arc<Mutex<Map>>,
}

impl GenerateFlightPlanSystem {
    pub fn new(event_bus: Sender<Event>, map: Arc<Mutex<Map>>) -> Self {
        let rng = thread_rng();

        Self {
            event_bus,
            rng,
            map,
        }
    }
}

impl GenerateFlightPlanSystem {
    fn generate_random_plan(&mut self, travelled_route: &TravelledRoute) -> FlightPlan {
        let mut flight_plan = Vec::new();

        let mut reversed_route = travelled_route.get().iter().rev();

        let mut current_node = reversed_route
            .next()
            .expect("travelled route must include the starting location")
            .clone();
        let mut previous_node = reversed_route.next().cloned();
        let mut next_node;

        for _ in 0..FLIGHT_PLAN_LENGTH {
            next_node = self.pick_next_tile(&current_node, &previous_node);
            flight_plan.push(next_node.clone());

            previous_node = Some(current_node);
            current_node = next_node;
        }

        FlightPlan::new(flight_plan.iter().rev().cloned().collect())
    }

    fn pick_next_tile(
        &mut self,
        current_tile: &Arc<Node>,
        previous_tile: &Option<Arc<Node>>,
    ) -> Arc<Node> {
        let map = self.map.lock();

        let potential_tiles: Vec<Arc<Node>> = map
            .grid()
            .neighbors(current_tile.longitude(), current_tile.latitude())
            .into_iter()
            .filter(|node| Some(node) != previous_tile.as_ref())
            .filter(|node| !node.is_restricted())
            .collect();

        potential_tiles
            .choose(&mut self.rng)
            .expect("failed to choose random tile")
            .clone()
    }
}

impl System for GenerateFlightPlanSystem {
    fn update(&mut self, world: &mut hecs::World, _delta: f32) {
        for (_entity, (airplane_id, flight_plan, travelled_route)) in
            world.query_mut::<(&AirplaneId, &mut FlightPlan, &TravelledRoute)>()
        {
            if flight_plan.get().len() > 1 {
                continue;
            }

            let new_flight_plan = self.generate_random_plan(travelled_route);
            *flight_plan = new_flight_plan.clone();

            self.event_bus
                .send(Event::FlightPlanUpdated(
                    airplane_id.clone(),
                    new_flight_plan,
                ))
                .expect("failed to send FlightPlanUpdated event");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<GenerateFlightPlanSystem>();
    }
}
