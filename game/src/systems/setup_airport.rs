use bevy::prelude::*;

use crate::components::Tag;
use crate::map::Map;
use crate::rendering::RenderLayer;

pub fn setup_airport(map: Res<Map>, mut commands: Commands) {
    for airport in map.airports() {
        let airport_vec3 = airport.node().as_vec3(RenderLayer::Airport.z());

        let color = match airport.tag() {
            Tag::Blue => Color::BLUE,
            Tag::Red => Color::RED,
        };

        commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: airport_vec3,
                scale: Vec3::new(24.0, 24.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
