use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::{
    Airplane, AirplaneIdGenerator, Collider, FlightPlan, Location, Speed, Tag, TravelledRoute,
};
use crate::event::{Event, EventBus};
use crate::map::{Direction, Node, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};
use crate::rendering::RenderLayer;

// Planes are spawned this many nodes outside the map area
const SPAWN_OFFSET: i32 = 3;

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
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut airplane_id_generator: Local<AirplaneIdGenerator>,
    event_bus: Local<EventBus>,
) {
    let mut rng = thread_rng();

    let texture_handle = asset_server.load("sprites/airplanes.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 2, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    if timer.0.tick(time.delta()).just_finished() {
        let (spawn, first_node) = random_spawn();
        let spawn_point = spawn.as_point();

        let airplane_id = airplane_id_generator.generate();
        let travelled_route = TravelledRoute::new(vec![spawn]);
        let flight_plan = FlightPlan::new(vec![first_node]);

        let direction = Direction::between(&first_node.as_point(), &spawn_point);

        let tag = if rng.gen_bool(0.5) {
            Tag::Blue
        } else {
            Tag::Red
        };

        let color_offset = match tag {
            Tag::Blue => 0,
            Tag::Red => 1,
        };

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(
                        spawn_point.x(),
                        spawn_point.y(),
                        RenderLayer::Airplane.z(),
                    ),
                    rotation: Quat::from_rotation_z(direction.to_degree().to_radians()),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: color_offset,
                    custom_size: Some(Vec2::new(32.0, 32.0)),
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

fn random_spawn() -> (Node, Node) {
    let mut rng = thread_rng();

    let (first_node, spawn) = match rng.gen_range(0u32..4u32) {
        0 => {
            let x = rng.gen_range(MAP_WIDTH_RANGE);
            (
                Node::unrestricted(x, *MAP_HEIGHT_RANGE.end()),
                Node::unrestricted(x, *MAP_HEIGHT_RANGE.end() + SPAWN_OFFSET),
            )
        }
        1 => {
            let y = rng.gen_range(MAP_HEIGHT_RANGE);
            (
                Node::unrestricted(*MAP_WIDTH_RANGE.end(), y),
                Node::unrestricted(*MAP_WIDTH_RANGE.end() + SPAWN_OFFSET, y),
            )
        }
        2 => {
            let x = rng.gen_range(MAP_WIDTH_RANGE);
            (
                Node::unrestricted(x, *MAP_HEIGHT_RANGE.start()),
                Node::unrestricted(x, *MAP_HEIGHT_RANGE.start() - SPAWN_OFFSET),
            )
        }
        _ => {
            let y = rng.gen_range(MAP_HEIGHT_RANGE);
            (
                Node::unrestricted(*MAP_WIDTH_RANGE.start(), y),
                Node::unrestricted(*MAP_WIDTH_RANGE.start() - SPAWN_OFFSET, y),
            )
        }
    };

    (spawn, first_node)
}
