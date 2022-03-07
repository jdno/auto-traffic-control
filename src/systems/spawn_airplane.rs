use bevy::prelude::*;
use geo::point;

use crate::components::{Airplane, FlightPlan, Movement};
use crate::map::{Direction, Tile, MAP_WIDTH_RANGE};
use crate::TILE_SIZE;

pub fn spawn_airplane(mut commands: Commands) {
    let spawn = point!(x: TILE_SIZE * MAP_WIDTH_RANGE.start(), y: 0);
    let flight_plan = (*MAP_WIDTH_RANGE.start()..=0)
        .map(|x| Tile::new(x, 0))
        .collect();

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(spawn.x() as f32, spawn.y() as f32, 2.0),
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
        .insert(Movement::new(32.0, Direction::East));
}
