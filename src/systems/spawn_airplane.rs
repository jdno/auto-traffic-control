use bevy::prelude::*;

use crate::components::{Airplane, FlightPlan, Speed};
use crate::map::{route_between, Tile, MAP_WIDTH_RANGE};

pub fn spawn_airplane(mut commands: Commands) {
    let spawn = Tile::new(*MAP_WIDTH_RANGE.start(), 0);
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
