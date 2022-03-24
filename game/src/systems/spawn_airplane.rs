use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::{
    Airplane, AirplaneIdGenerator, Collider, Location, Speed, Tag, TravelledRoute, AIRPLANE_SIZE,
};
use crate::map::{Map, Node, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};
use crate::{generate_random_plan, Event, EventBus};

pub struct SpawnTimer(Timer);

impl SpawnTimer {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
}

pub fn spawn_airplane(
    map: Res<Map>,
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut airplane_id_generator: Local<AirplaneIdGenerator>,
    event_bus: Local<EventBus>,
) {
    let mut rng = thread_rng();

    if timer.0.tick(time.delta()).just_finished() {
        let spawn = random_spawn();
        let spawn_point = spawn.as_point();

        let airplane_id = airplane_id_generator.generate();

        let travelled_route = TravelledRoute::new(vec![spawn]);
        let flight_plan = generate_random_plan(&travelled_route, &map);

        let tag = if rng.gen_bool(0.5) {
            Tag::Blue
        } else {
            Tag::Red
        };

        let color = match tag {
            Tag::Blue => Color::ALICE_BLUE,
            Tag::Red => Color::SALMON,
        };

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(spawn_point.x(), spawn_point.y(), 2.0),
                    scale: Vec3::new(AIRPLANE_SIZE, AIRPLANE_SIZE, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Airplane)
            .insert(airplane_id.clone())
            .insert(Collider)
            .insert(flight_plan.clone())
            .insert(Speed::new(32.0))
            .insert(tag)
            .insert(travelled_route);

        event_bus
            .sender()
            .send(Event::AirplaneDetected(
                airplane_id,
                Location::from(&spawn),
                flight_plan,
                tag,
            ))
            .expect("failed to send event"); // TODO: Handle error
    }
}

fn random_spawn() -> Node {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0u32..4u32) {
        0 => {
            let x = rng.gen_range(MAP_WIDTH_RANGE);
            Node::unrestricted(x, *MAP_HEIGHT_RANGE.end())
        }
        1 => {
            let y = rng.gen_range(MAP_HEIGHT_RANGE);
            Node::unrestricted(*MAP_WIDTH_RANGE.end(), y)
        }
        2 => {
            let x = rng.gen_range(MAP_WIDTH_RANGE);
            Node::unrestricted(x, *MAP_HEIGHT_RANGE.start())
        }
        _ => {
            let y = rng.gen_range(MAP_HEIGHT_RANGE);
            Node::unrestricted(*MAP_WIDTH_RANGE.start(), y)
        }
    }
}
