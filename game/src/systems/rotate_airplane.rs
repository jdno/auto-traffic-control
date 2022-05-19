use bevy::prelude::*;

use crate::components::{FlightPlan, TravelledRoute};
use crate::map::Direction;

pub fn rotate_airplane(mut query: Query<(&mut FlightPlan, &mut Transform, &mut TravelledRoute)>) {
    for (flight_plan, mut transform, travelled_route) in query.iter_mut() {
        let last_point = travelled_route
            .get()
            .last()
            .expect("travelled route must not be empty")
            .as_point();

        let next_point = match flight_plan.next() {
            Some(node) => node.as_point(),
            None => continue,
        };

        let direction = Direction::between(&next_point, &last_point);
        transform.rotation = Quat::from_rotation_z(direction.to_degree().to_radians());
    }
}
