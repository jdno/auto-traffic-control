use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

use hecs::World;
use parking_lot::Mutex;
use rand::prelude::*;
use time::{Duration, Instant};

use crate::bus::{Event, Sender};
use crate::component::{FlightPlan, Tag, TravelledRoute};
use crate::map::{Location, Map, Node, MAP_BORDER_WIDTH};
use crate::system::System;
use crate::util::AirplaneIdGenerator;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Side {
    North,
    East,
    South,
    West,
}

impl Side {
    fn random(rng: &mut ThreadRng) -> Self {
        match rng.gen_range(0..=3) {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            _ => Self::West,
        }
    }
}

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
        let side = Side::random(&mut self.rng);

        let start_node = self.start_node(side);
        let first_node = self.first_node(side, &start_node);

        Spawn {
            start_node,
            first_node,
        }
    }

    fn start_node(&mut self, side: Side) -> Arc<Node> {
        let map = self.map.lock();

        let (x, y) = match side {
            Side::North => (
                self.rng
                    .gen_range(MAP_BORDER_WIDTH..(map.width() - MAP_BORDER_WIDTH)),
                0,
            ),
            Side::East => (
                map.width() - 1,
                self.rng
                    .gen_range(MAP_BORDER_WIDTH..(map.height() - MAP_BORDER_WIDTH)),
            ),
            Side::South => (
                self.rng
                    .gen_range(MAP_BORDER_WIDTH..(map.width() - MAP_BORDER_WIDTH)),
                map.height() - 1,
            ),
            Side::West => (
                0,
                self.rng
                    .gen_range(MAP_BORDER_WIDTH..(map.height() - MAP_BORDER_WIDTH)),
            ),
        };

        map.grid()
            .get(x, y)
            .expect("failed to get start node for airplane")
            .clone()
    }

    fn first_node(&self, side: Side, start_node: &Arc<Node>) -> Arc<Node> {
        let x = start_node.longitude();
        let y = start_node.latitude();

        let (x, y) = match side {
            Side::North => (x, y + MAP_BORDER_WIDTH),
            Side::East => (x - MAP_BORDER_WIDTH, y),
            Side::South => (x, y - MAP_BORDER_WIDTH),
            Side::West => (x + MAP_BORDER_WIDTH, y),
        };

        self.map
            .lock()
            .grid()
            .get(x, y)
            .expect("failed to get first node for flight plan")
            .clone()
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

    use super::*;

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
