use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

use hecs::World;
use parking_lot::Mutex;
use rand::prelude::*;
use time::{Duration, Instant};

use crate::bus::{Event, Sender};
use crate::component::{FlightPlan, Tag, TravelledRoute};
use crate::map::{Location, Map, Node};
use crate::system::System;
use crate::util::AirplaneIdGenerator;

struct Spawn {
    start_node: Arc<Node>,
    first_node: Arc<Node>,
}

#[derive(Clone, Debug)]
pub struct SpawnAirplaneSystem {
    event_bus: Sender<Event>,
    rng: ThreadRng,

    interval: Duration,
    last_spawn: Instant,

    airplane_id_generator: AirplaneIdGenerator,
    map: Arc<Mutex<Map>>,
}

impl SpawnAirplaneSystem {
    pub fn new(event_bus: Sender<Event>, map: Arc<Mutex<Map>>, interval: Duration) -> Self {
        Self {
            event_bus,
            rng: thread_rng(),

            interval,
            last_spawn: Instant::now(),

            airplane_id_generator: AirplaneIdGenerator::default(),
            map,
        }
    }

    fn random_spawn(&mut self) -> Spawn {
        let start_node = self.start_node();
        let first_node = self.first_node(&start_node);

        Spawn {
            start_node,
            first_node,
        }
    }

    fn start_node(&mut self) -> Arc<Node> {
        self.map
            .lock()
            .spawns()
            .choose(&mut self.rng)
            .expect("failed to get random spawn")
            .clone()
    }

    fn first_node(&self, start_node: &Arc<Node>) -> Arc<Node> {
        let mut x = start_node.longitude() as i32;
        let mut y = start_node.latitude() as i32;

        let map = self.map.lock();
        let width = (map.width() - 1) as i32;
        let height = (map.height() - 1) as i32;

        let (dx, dy) = match (x, y) {
            (0, _) => (1, 0),
            (_, 0) => (0, 1),
            (_, _) if x == width => (-1, 0),
            (_, _) if y == height => (0, -1),
            _ => panic!("spawn node is not on the edge of the map"),
        };

        while x <= width && y <= height {
            let node = map
                .grid()
                .get(x as u32, y as u32)
                .expect("failed to get node from grid");

            if !node.is_restricted() {
                return node.clone();
            }

            x += dx;
            y += dy;
        }

        panic!("failed to find a valid first node")
    }

    fn random_tag(&mut self) -> Tag {
        let tag = self.rng.gen_bool(0.5);

        if tag {
            Tag::Blue
        } else {
            Tag::Red
        }
    }
}

impl Display for SpawnAirplaneSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpawnAirplaneSystem")
    }
}

impl System for SpawnAirplaneSystem {
    fn update(&mut self, world: &mut World, _delta: f32) {
        if self.last_spawn + self.interval >= Instant::now() {
            return;
        }

        let spawn = self.random_spawn();

        let id = self.airplane_id_generator.generate();
        let location: Location = spawn.start_node.deref().into();
        let flight_plan = FlightPlan::new(vec![spawn.first_node]);
        let travelled_route = TravelledRoute::new(vec![spawn.start_node]);
        let tag = self.random_tag();

        world.spawn((
            id.clone(),
            location,
            flight_plan.clone(),
            travelled_route,
            tag,
        ));

        self.event_bus
            .send(Event::AirplaneDetected(id, location, flight_plan, tag))
            .expect("failed to send AirplaneDetected event");
    }
}

#[cfg(test)]
mod tests {
    use crate::bus::channel;
    use crate::prelude::AirplaneId;

    use super::*;

    #[test]
    fn no_spawn() {
        let (sender, mut receiver) = channel(1);
        let mut world = World::new();

        let mut system = SpawnAirplaneSystem {
            event_bus: sender,
            rng: thread_rng(),
            interval: Duration::seconds(2),
            last_spawn: Instant::now() - Duration::seconds(1),
            airplane_id_generator: AirplaneIdGenerator::default(),
            map: Map::test(),
        };

        system.update(&mut world, 0.0);

        assert!(receiver.try_recv().is_err());
    }

    #[test]
    fn spawn_airplane() {
        let (sender, mut receiver) = channel(1);
        let mut world = World::new();

        let mut system = SpawnAirplaneSystem {
            event_bus: sender,
            rng: thread_rng(),
            interval: Duration::seconds(2),
            last_spawn: Instant::now() - Duration::seconds(3),
            airplane_id_generator: AirplaneIdGenerator::default(),
            map: Map::test(),
        };

        system.update(&mut world, 0.0);

        let event = receiver.try_recv().unwrap();
        let (airplane_id, location, flight_plan) = match event {
            Event::AirplaneDetected(airplane_id, location, flight_plan, _) => {
                (airplane_id, location, flight_plan)
            }
            _ => panic!("unexpected event"),
        };

        assert_eq!(AirplaneId::new("AT-0001".into()), airplane_id);
        assert_eq!(Location::new(192.0, 0.0), location);
        assert_eq!(
            FlightPlan::new(vec![Arc::new(Node::new(3, 1, false))]),
            flight_plan
        );
    }

    #[test]
    fn trait_display() {
        let (sender, _) = channel(1);

        let system = SpawnAirplaneSystem::new(
            sender,
            Arc::new(Mutex::new(Map::default())),
            Duration::seconds(1),
        );

        assert_eq!(format!("{}", system), "SpawnAirplaneSystem");
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<SpawnAirplaneSystem>();
    }
}
