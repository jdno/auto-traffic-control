use bevy::prelude::*;

use crate::components::{Movement, Position};
use crate::map::Coordinate;

pub fn move_entities(
    time: Res<Time>,
    mut query: Query<(&mut Position, &Movement, &mut Transform)>,
) {
    for (mut position, movement, mut transform) in query.iter_mut() {
        let distance = movement.speed() * time.delta().as_secs_f32();
        let distance_as_vector = movement.direction().to_vec3() * distance;

        let new_translation = transform.translation + distance_as_vector;
        let new_position = Position::new(Coordinate::from(&new_translation));

        *position = new_position;
        transform.translation = new_translation;
    }
}
