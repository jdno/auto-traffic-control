use bevy::prelude::*;

use crate::map::Map;
use crate::rendering::RenderLayer;

pub fn setup_grid(map: Res<Map>, mut commands: Commands) {
    for node in map
        .routing_grid()
        .iter()
        .filter(|node| !node.is_restricted())
    {
        let point = node.as_point();

        commands.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(point.x(), point.y(), RenderLayer::RoutingGrid.z()),
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
