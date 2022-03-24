use bevy::prelude::*;

use crate::map::Map;

pub fn setup_grid(map: Res<Map>, mut commands: Commands) {
    for node in map
        .routing_grid()
        .iter()
        .filter(|node| !node.is_restricted())
    {
        let point = node.as_point();

        commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(point.x() as f32, point.y() as f32, 0.0),
                scale: Vec3::new(2.0, 2.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
