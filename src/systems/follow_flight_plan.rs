use bevy::prelude::*;
use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::point;

use crate::components::{FlightPlan, Speed};
use crate::map::Direction;

pub fn follow_flight_plan(
    time: Res<Time>,
    mut query: Query<(&mut FlightPlan, &Speed, &mut Transform)>,
) {
    for (mut flight_plan, speed, mut transform) in query.iter_mut() {
        let distance = speed.get() * time.delta().as_secs_f32();

        fly(&mut transform.translation, &mut flight_plan, distance);
    }
}

fn fly(current_position: &mut Vec3, flight_plan: &mut FlightPlan, travelled_distance: f32) {
    if let Some(next_tile) = flight_plan.get().iter().last() {
        let current_point = point!(x: current_position.x, y: current_position.y);
        let next_point = next_tile.as_point();

        let distance_between_points = current_point.euclidean_distance(&next_point);

        if travelled_distance >= distance_between_points {
            *current_position = next_tile.as_vec3(current_position.z);
            flight_plan.get_mut().pop().unwrap();

            fly(
                current_position,
                flight_plan,
                travelled_distance - distance_between_points,
            );
        } else {
            let direction = Direction::between(&next_point, &current_point);
            *current_position += direction.to_vec3() * travelled_distance;
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::components::FlightPlan;
    use crate::map::Tile;
    use crate::TILE_SIZE;

    use super::fly;

    #[test]
    fn fly_and_reach_next_node() {
        let mut current_position = Tile::new(0, 0).as_vec3(0.0);
        let mut flight_plan = FlightPlan::new(vec![Tile::new(1, 0)]);

        fly(&mut current_position, &mut flight_plan, TILE_SIZE as f32);

        assert_eq!(current_position, Vec3::new(TILE_SIZE as f32, 0.0, 0.0));
        assert_eq!(0, flight_plan.get().len());
    }

    #[test]
    fn fly_towards_next_node() {
        let mut current_position = Tile::new(0, 0).as_vec3(0.0);
        let mut flight_plan = FlightPlan::new(vec![Tile::new(1, 0)]);

        let movement_speed = (TILE_SIZE / 2) as f32;

        fly(&mut current_position, &mut flight_plan, movement_speed);

        assert_eq!(current_position, Vec3::new(movement_speed, 0.0, 0.0));
    }

    #[test]
    fn fly_past_node() {
        let mut current_position = Tile::new(0, 0).as_vec3(0.0);
        let mut flight_plan = FlightPlan::new(vec![Tile::new(2, 0), Tile::new(1, 0)]);

        let movement_speed = (TILE_SIZE + (TILE_SIZE / 2)) as f32;

        fly(&mut current_position, &mut flight_plan, movement_speed);

        assert_eq!(current_position, Vec3::new(movement_speed, 0.0, 0.0));
        assert_eq!(1, flight_plan.get().len());
    }

    #[test]
    fn fly_past_node_and_change_direction() {
        let mut current_position = Tile::new(0, 0).as_vec3(0.0);
        let mut flight_plan = FlightPlan::new(vec![Tile::new(1, 1), Tile::new(1, 0)]);

        let movement_speed = (TILE_SIZE + (TILE_SIZE / 2)) as f32;

        fly(&mut current_position, &mut flight_plan, movement_speed);

        assert_eq!(current_position, Vec3::new(32.0, 16.0, 0.0));
        assert_eq!(1, flight_plan.get().len());
    }
}
