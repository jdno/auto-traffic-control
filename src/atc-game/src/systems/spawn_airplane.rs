use bevy::prelude::*;
use rand::Rng;

use crate::components::{
    Airplane, AirplaneIdGenerator, Collider, Location, Speed, TravelledRoute, AIRPLANE_SIZE,
};
use crate::map::{Tile, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};
use crate::{generate_random_plan, Event, EventBus};

pub struct SpawnTimer(Timer);

impl SpawnTimer {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
}

pub fn spawn_airplane(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut airplane_id_generator: Local<AirplaneIdGenerator>,
    event_bus: Local<EventBus>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let spawn = random_spawn();
        let spawn_point = spawn.as_point();

        let airplane_id = airplane_id_generator.generate();

        let travelled_route = TravelledRoute::new(vec![spawn]);
        let flight_plan = generate_random_plan(&travelled_route);

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(spawn_point.x(), spawn_point.y(), 2.0),
                    scale: Vec3::new(AIRPLANE_SIZE, AIRPLANE_SIZE, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::RED,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Airplane)
            .insert(airplane_id.clone())
            .insert(Collider)
            .insert(flight_plan.clone())
            .insert(Speed::new(32.0))
            .insert(travelled_route);

        event_bus
            .sender()
            .send(Event::AirplaneDetected(
                airplane_id,
                Location::from(&spawn),
                flight_plan,
            ))
            .expect("failed to send event"); // TODO: Handle error
    }
}

fn random_spawn() -> Tile {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0u32..4u32) {
        0 => {
            let x = rng.gen_range(MAP_WIDTH_RANGE);
            Tile::new(x, *MAP_HEIGHT_RANGE.end())
        }
        1 => {
            let y = rng.gen_range(MAP_HEIGHT_RANGE);
            Tile::new(*MAP_WIDTH_RANGE.end(), y)
        }
        2 => {
            let x = rng.gen_range(MAP_WIDTH_RANGE);
            Tile::new(x, *MAP_HEIGHT_RANGE.start())
        }
        _ => {
            let y = rng.gen_range(MAP_HEIGHT_RANGE);
            Tile::new(*MAP_WIDTH_RANGE.start(), y)
        }
    }
}
