use hecs::World;
use std::ops::Deref;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::bus::{Event, Sender};
use crate::component::{AirplaneId, FlightPlan, Tag};
use crate::map::{Airport, Grid, Location, Map, Node};
use crate::system::System;

#[derive(Clone, Debug)]
pub struct DespawnAirplaneSystem {
    event_bus: Sender<Event>,
    map: Arc<Mutex<Map>>,
}

impl DespawnAirplaneSystem {
    pub fn new(event_bus: Sender<Event>, map: Arc<Mutex<Map>>) -> Self {
        Self { event_bus, map }
    }
}

impl System for DespawnAirplaneSystem {
    fn update(&mut self, world: &mut World, _delta: f32) {
        let mut landed_airplanes = Vec::new();
        let map = self.map.lock();

        for (entity, (airplane_id, location, flight_plan, tag)) in
            world.query_mut::<(&AirplaneId, &Location, &mut FlightPlan, &Tag)>()
        {
            for airport in map.airports() {
                if airport.location() != location {
                    continue;
                }

                if airport.tag() == *tag {
                    landed_airplanes.push(entity);

                    self.event_bus
                        .send(Event::AirplaneLanded(airplane_id.clone()))
                        .expect("failed to send AirplaneLanded event");
                } else {
                    let go_around_node = go_around_procedure(airport, map.grid());
                    *flight_plan = FlightPlan::new(vec![go_around_node]);

                    self.event_bus
                        .send(Event::LandingAborted(airplane_id.clone()))
                        .expect("failed to send LandingAborted event");

                    self.event_bus
                        .send(Event::FlightPlanUpdated(
                            airplane_id.clone(),
                            flight_plan.clone(),
                        ))
                        .expect("failed to send FlightPlanUpdated event");
                }
            }
        }
    }
}

fn go_around_procedure(airport: &Airport, grid: &Grid<Arc<Node>>) -> Arc<Node> {
    let neighbors = grid.neighbors(airport.node().longitude(), airport.node().longitude());

    let runway = neighbors
        .iter()
        .find(|node| !node.is_restricted())
        .expect("failed to find runway for airport");

    let direction = (runway.deref() - airport.node().deref()).as_tuple();

    let go_around = grid
        .get(
            (airport.node().longitude() as i32 + 2 * direction.0) as u32,
            (airport.node().latitude() as i32 + 2 * direction.1) as u32,
        )
        .expect("failed to find go around node for airport");

    go_around.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<DespawnAirplaneSystem>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<DespawnAirplaneSystem>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<DespawnAirplaneSystem>();
    }
}
