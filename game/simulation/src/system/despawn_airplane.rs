use std::ops::Deref;
use std::sync::Arc;

use hecs::World;
use parking_lot::Mutex;

use crate::bus::{Event, Sender};
use crate::component::{AirplaneId, FlightPlan, Tag};
use crate::map::{Airport, Grid, Location, Map, Node};
use crate::system::System;

#[derive(Clone, Debug)]
pub struct DespawnAirplaneSystem {
    event_bus: Sender<Event>,
    map: Arc<Mutex<Map>>,
    score: Arc<Mutex<u32>>,
}

impl DespawnAirplaneSystem {
    pub fn new(event_bus: Sender<Event>, map: Arc<Mutex<Map>>, score: Arc<Mutex<u32>>) -> Self {
        Self {
            event_bus,
            map,
            score,
        }
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
                    *self.score.lock() += 1;

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
    use crate::bus::channel;

    use super::*;

    #[test]
    fn no_landings() {
        let (sender, mut receiver) = channel(1);

        let map = Map::test();
        let score = Arc::new(Mutex::new(0));

        let mut system = DespawnAirplaneSystem::new(sender, map, score.clone());

        let mut world = World::new();
        world.spawn((
            AirplaneId::default(),
            Location::new(0.0, 0.0),
            FlightPlan::new(vec![Arc::new(Node::new(0, 0, false))]),
            Tag::Blue,
        ));

        system.update(&mut world, 0.0);

        assert!(receiver.try_recv().is_err());
        assert_eq!(0, *score.lock());
    }

    #[test]
    fn land_airplane() {
        let (sender, mut receiver) = channel(1);

        let map = Map::test();
        let airport = map.lock().airports().first().unwrap().clone();
        let score = Arc::new(Mutex::new(0));
        let airplane_id = AirplaneId::default();

        let mut system = DespawnAirplaneSystem::new(sender, map, score.clone());

        let mut world = World::new();
        world.spawn((
            airplane_id.clone(),
            Location::new(airport.location().x(), airport.location().y()),
            FlightPlan::new(Vec::new()),
            airport.tag(),
        ));

        system.update(&mut world, 0.0);

        let event = receiver.try_recv().unwrap();

        assert_eq!(Event::AirplaneLanded(airplane_id), event);
        assert_eq!(1, *score.lock());
    }

    #[test]
    fn go_around() {
        let (sender, mut receiver) = channel(2);

        let map = Map::test();
        let airport = map.lock().airports().first().unwrap().clone();
        let score = Arc::new(Mutex::new(0));
        let airplane_id = AirplaneId::default();

        let mut system = DespawnAirplaneSystem::new(sender, map, score.clone());

        let mut world = World::new();
        world.spawn((
            airplane_id.clone(),
            Location::new(airport.location().x(), airport.location().y()),
            FlightPlan::new(Vec::new()),
            Tag::Red,
        ));

        system.update(&mut world, 0.0);

        let event = receiver.try_recv().unwrap();

        assert_eq!(Event::LandingAborted(airplane_id), event);
        assert_eq!(0, *score.lock());
    }

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
