use bevy::prelude::*;

use crate::map::{Node, MAP_HEIGHT_RANGE, MAP_WIDTH_RANGE};

pub fn setup_grid(mut commands: Commands) {
    for y in MAP_HEIGHT_RANGE {
        for x in MAP_WIDTH_RANGE {
            let node = Node::new(x, y);
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
}
