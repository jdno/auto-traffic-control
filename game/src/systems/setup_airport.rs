use bevy::prelude::*;

use crate::map::Map;

pub fn setup_airport(map: Res<Map>, mut commands: Commands) {
    let airport_vec3 = map.airport().as_vec3(1.0);

    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: airport_vec3,
            scale: Vec3::new(24.0, 24.0, 1.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::SEA_GREEN,
            ..Default::default()
        },
        ..Default::default()
    });
}
