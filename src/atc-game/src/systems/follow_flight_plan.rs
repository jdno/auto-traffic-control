use bevy::prelude::*;
use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::point;

use crate::components::{AirplaneId, FlightPlan, Location, Speed};
use crate::map::Direction;
use crate::{Event, EventBus};

pub fn follow_flight_plan(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &AirplaneId, &mut FlightPlan, &Speed, &mut Transform)>,
    event_bus: Local<EventBus>,
) {
    for (entity, airplane_id, mut flight_plan, speed, mut transform) in query.iter_mut() {
        let distance = speed.get() * time.delta().as_secs_f32();

        let did_update_flight_plan = fly(&mut transform.translation, &mut flight_plan, distance);
        let mut airplane_landed = false;

        event_bus
            .sender()
            .send(Event::AirplaneMoved(
                airplane_id.clone(),
                Location::from(&transform),
            ))
            .expect("failed to send event"); // TODO: Handle error

        // Airplane reached the airport
        if transform.translation == Vec3::new(0.0, 0.0, 2.0) {
            airplane_landed = true;

            commands.entity(entity).despawn();

            event_bus
                .sender()
                .send(Event::AirplaneLanded(airplane_id.clone()))
                .expect("failed to send event"); // TODO: Handle error
        }

        if did_update_flight_plan && !airplane_landed {
            event_bus
                .sender()
                .send(Event::FlightPlanUpdated(
                    airplane_id.clone(),
                    flight_plan.clone(),
                ))
                .expect("failed to send event"); // TODO: Handle error
        }
    }
}

fn fly(current_position: &mut Vec3, flight_plan: &mut FlightPlan, travelled_distance: f32) -> bool {
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

            return true;
        } else {
            let direction = Direction::between(&next_point, &current_point);
            *current_position += direction.to_vec3() * travelled_distance;
        }
    }

    false
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
