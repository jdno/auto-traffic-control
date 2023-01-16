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
    fn generate_random_plan(
        &mut self,
        current_flight_plan: &mut FlightPlan,
        travelled_route: &TravelledRoute,
    ) -> FlightPlan {
        let mut previous_node = travelled_route
            .get()
            .last()
            .expect("travelled route must not be empty")
            .clone();

        let mut current_node = current_flight_plan
            .get()
            .last()
            .expect("flight plan must not be empty")
            .clone();

        let mut flight_plan = vec![current_node.clone()];
        let mut next_node;

        for _ in 0..FLIGHT_PLAN_LENGTH - 1 {
            next_node = self.pick_next_tile(&current_node, &previous_node);
            flight_plan.push(next_node.clone());

            previous_node = current_node;
            current_node = next_node;
        }

        FlightPlan::new(flight_plan.iter().rev().cloned().collect())
    }

    fn pick_next_tile(&mut self, current_tile: &Arc<Node>, previous_tile: &Arc<Node>) -> Arc<Node> {
        let map = self.map.lock();

        let potential_tiles: Vec<Arc<Node>> = map
            .grid()
            .neighbors(current_tile.longitude(), current_tile.latitude())
            .into_iter()
            .filter(|node| node != previous_tile)
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

            let new_flight_plan = self.generate_random_plan(flight_plan, travelled_route);
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
    use hecs::World;

    use crate::bus::channel;
    use crate::component::Tag;

    use super::*;

    #[test]
    fn no_flight_plan() {
        let (sender, mut receiver) = channel(1);

        let map = Map::test();
        let mut world = World::new();

        let mut system = GenerateFlightPlanSystem::new(sender, map);

        world.spawn((
            AirplaneId::default(),
            FlightPlan::new(vec![
                Arc::new(Node::new(2, 0, false)),
                Arc::new(Node::new(1, 0, false)),
            ]),
            TravelledRoute::new(vec![Arc::new(Node::new(0, 0, false))]),
            Tag::Blue,
        ));

        system.update(&mut world, 0.0);

        assert!(receiver.try_recv().is_err());
    }

    #[test]
    fn generate_flight_plan() {
        let (sender, mut receiver) = channel(1);

        let map = Map::test();
        let mut world = World::new();

        let mut system = GenerateFlightPlanSystem::new(sender, map);

        world.spawn((
            AirplaneId::default(),
            FlightPlan::new(vec![Arc::new(Node::new(2, 0, false))]),
            TravelledRoute::new(vec![Arc::new(Node::new(1, 0, false))]),
            Tag::Blue,
        ));

        system.update(&mut world, 0.0);

        let event = receiver.try_recv().unwrap();

        assert!(matches!(event, Event::FlightPlanUpdated(_, _)));
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<GenerateFlightPlanSystem>();
    }
}
