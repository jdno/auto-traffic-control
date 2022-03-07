use bevy::prelude::*;

use crate::components::Movement;

pub fn move_entities(time: Res<Time>, mut query: Query<(&Movement, &mut Transform)>) {
    for (movement, mut transform) in query.iter_mut() {
        let distance = movement.speed() * time.delta().as_secs_f32();
        let distance_as_vector = movement.direction().to_vec3() * distance;

        transform.translation += distance_as_vector;
    }
}
