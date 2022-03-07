use bevy::prelude::*;

pub fn setup_airport(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
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
