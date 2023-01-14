use parking_lot::Mutex;
use std::sync::Arc;

use rand::prelude::*;
use time::{Duration, Instant};

use crate::behavior::Updateable;
use crate::bus::{Command, Event, Receiver, Sender, COMMAND_BUS, EVENT_BUS};
use crate::component::Tag;
use crate::entity::Airplane;
use crate::map::{Map, Node, MAP_BORDER_WIDTH};
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

#[derive(Clone, PartialEq, Debug, Default)]
struct Spawn {
    start_node: Arc<Node>,
    first_node: Arc<Node>,
}

#[derive(Debug)]
pub struct Spawner {
    command_bus: Receiver<Command>,
    event_bus: Sender<Event>,

    rng: ThreadRng,

    airplane_id_generator: AirplaneIdGenerator,
    map: Arc<Mutex<Map>>,
    width: u32,
    height: u32,

    interval: Duration,
    last_spawn: Instant,
}

impl Spawner {
    pub fn new(map: Arc<Mutex<Map>>, interval: Duration) -> Self {
        let width = map.lock().width();
        let height = map.lock().height();

        Self {
            command_bus: COMMAND_BUS.1.resubscribe(),
            event_bus: EVENT_BUS.0.clone(),

            rng: thread_rng(),

            airplane_id_generator: AirplaneIdGenerator::default(),
            map,
            width,
            height,

            interval,
            last_spawn: Instant::now(),
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
        let (x, y) = match side {
            Side::North => (
                self.rng
                    .gen_range(MAP_BORDER_WIDTH..(self.width - MAP_BORDER_WIDTH)),
                0,
            ),
            Side::East => (
                self.width - 1,
                self.rng
                    .gen_range(MAP_BORDER_WIDTH..(self.height - MAP_BORDER_WIDTH)),
            ),
            Side::South => (
                self.rng
                    .gen_range(MAP_BORDER_WIDTH..(self.width - MAP_BORDER_WIDTH)),
                self.height - 1,
            ),
            Side::West => (
                0,
                self.rng
                    .gen_range(MAP_BORDER_WIDTH..(self.height - MAP_BORDER_WIDTH)),
            ),
        };

        self.map
            .lock()
            .grid()
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

impl Updateable for Spawner {
    fn update(&mut self, _delta: f32) {
        if self.last_spawn + self.interval < Instant::now() {
            let spawn = self.random_spawn();

            let airplane = Airplane::new(
                self.command_bus.resubscribe(),
                self.event_bus.clone(),
                self.airplane_id_generator.generate(),
                self.random_tag(),
                spawn.start_node,
                spawn.first_node,
            );

            self.map.lock().airplanes_mut().push(airplane);
        }
    }
}
