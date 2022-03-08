use bevy::prelude::*;
use rand::Rng;

use crate::components::{Airplane, FlightPlan, Speed};
use crate::map::{route_between, Tile, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};

pub fn spawn_airplane(mut commands: Commands) {
    let spawn = random_spawn();
    let spawn_point = spawn.as_point();

    let airport = Tile::new(0, 0);
    let flight_plan = route_between(&spawn, &airport);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(spawn_point.x(), spawn_point.y(), 2.0),
                scale: Vec3::new(8.0, 8.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::BLUE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Airplane)
        .insert(FlightPlan::new(flight_plan))
        .insert(Speed::new(32.0));
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
