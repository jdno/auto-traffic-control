use bevy::prelude::*;
use geo::point;
use rand::{thread_rng, Rng};

use crate::components::{
    Airplane, AirplaneIdGenerator, Collider, FlightPlan, Location, Speed, Tag, TravelledRoute,
};
use crate::event::{Event, EventBus};
use crate::map::{Direction, Node, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};
use crate::rendering::RenderLayer;
use crate::TILE_SIZE;

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

    let texture_handle = asset_server.load("sprites/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 8, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    if timer.0.tick(time.delta()).just_finished() {
        let (spawn, entry) = random_spawn();
        let spawn_point = point!(x: spawn.x, y: spawn.y);

        let airplane_id = airplane_id_generator.generate();
        let flight_plan = FlightPlan::new(vec![entry]);

        let direction = Direction::between(&entry.as_point(), &spawn_point);

        let tag = if rng.gen_bool(0.5) {
            Tag::Blue
        } else {
            Tag::Red
        };

        let color_offset = match tag {
            Tag::Blue => 10,
            Tag::Red => 18,
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
                sprite: TextureAtlasSprite::new(color_offset),
                ..Default::default()
            })
            .insert(Airplane)
            .insert(airplane_id.clone())
            .insert(Collider)
            .insert(flight_plan.clone())
            .insert(Speed::new(32.0))
            .insert(tag)
            .insert(TravelledRoute::new(Vec::new()));

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

fn random_spawn() -> (Vec3, Node) {
    let mut rng = rand::thread_rng();

    let (entry, direction) = match rng.gen_range(0u32..4u32) {
        0 => {
            let x = rng.gen_range(MAP_WIDTH_RANGE);
            (
                Node::unrestricted(x, *MAP_HEIGHT_RANGE.end()),
                Direction::South,
            )
        }
        1 => {
            let y = rng.gen_range(MAP_HEIGHT_RANGE);
            (
                Node::unrestricted(*MAP_WIDTH_RANGE.end(), y),
                Direction::West,
            )
        }
        2 => {
            let x = rng.gen_range(MAP_WIDTH_RANGE);
            (
                Node::unrestricted(x, *MAP_HEIGHT_RANGE.start()),
                Direction::North,
            )
        }
        _ => {
            let y = rng.gen_range(MAP_HEIGHT_RANGE);
            (
                Node::unrestricted(*MAP_WIDTH_RANGE.start(), y),
                Direction::East,
            )
        }
    };

    let offset = direction.to_vec3() * Vec3::splat(-3.0 * TILE_SIZE as f32);
    let spawn = entry.as_vec3(RenderLayer::Airplane.z()) + offset;

    (spawn, entry)
}
