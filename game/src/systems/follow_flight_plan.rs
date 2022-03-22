use bevy::prelude::*;
use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::point;

use crate::components::{AirplaneId, FlightPlan, Location, Speed, TravelledRoute};
use crate::event::{Event, EventBus};
use crate::map::Direction;

pub fn follow_flight_plan(
    time: Res<Time>,
    mut query: Query<(
        &AirplaneId,
        &mut FlightPlan,
        &Speed,
        &mut Transform,
        &mut TravelledRoute,
    )>,
    event_bus: Local<EventBus>,
) {
    for (airplane_id, mut flight_plan, speed, mut transform, mut travelled_route) in
        query.iter_mut()
    {
        let distance = speed.get() * time.delta().as_secs_f32();

        let did_update_flight_plan = fly(
            &mut transform.translation,
            &mut flight_plan,
            &mut travelled_route,
            distance,
        );

        event_bus
            .sender()
            .send(Event::AirplaneMoved(
                airplane_id.clone(),
                Location::from(&transform),
            ))
            .expect("failed to send event"); // TODO: Handle error

        if did_update_flight_plan && !flight_plan.get().is_empty() {
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

fn fly(
    current_position: &mut Vec3,
    flight_plan: &mut FlightPlan,
    travelled_route: &mut TravelledRoute,
    travelled_distance: f32,
) -> bool {
    if let Some(next_tile) = flight_plan.get().iter().last() {
        let current_point = point!(x: current_position.x, y: current_position.y);
        let next_point = next_tile.as_point();

        let distance_between_points = current_point.euclidean_distance(&next_point);

        if travelled_distance >= distance_between_points {
            *current_position = next_tile.as_vec3(current_position.z);

            let node = flight_plan.get_mut().pop().unwrap();
            travelled_route.get_mut().push(node);

            fly(
                current_position,
                flight_plan,
                travelled_route,
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

    use crate::components::{FlightPlan, TravelledRoute};
    use crate::map::Node;
    use crate::TILE_SIZE;

    use super::fly;

    #[test]
    fn fly_and_reach_next_node() {
        let mut current_position = Node::new(0, 0).as_vec3(0.0);
        let mut flight_plan = FlightPlan::new(vec![Node::new(1, 0)]);
        let mut travelled_route = TravelledRoute::new(vec![Node::new(0, 0)]);

        fly(
            &mut current_position,
            &mut flight_plan,
            &mut travelled_route,
            TILE_SIZE as f32,
        );

        assert_eq!(current_position, Vec3::new(TILE_SIZE as f32, 0.0, 0.0));
        assert_eq!(0, flight_plan.get().len());
        assert_eq!(2, travelled_route.get().len());
    }

    #[test]
    fn fly_towards_next_node() {
        let mut current_position = Node::new(0, 0).as_vec3(0.0);
        let mut flight_plan = FlightPlan::new(vec![Node::new(1, 0)]);
        let mut travelled_route = TravelledRoute::new(vec![Node::new(0, 0)]);

        let movement_speed = (TILE_SIZE / 2) as f32;

        fly(
            &mut current_position,
            &mut flight_plan,
            &mut travelled_route,
            movement_speed,
        );

        assert_eq!(current_position, Vec3::new(movement_speed, 0.0, 0.0));
        assert_eq!(1, travelled_route.get().len());
    }

    #[test]
    fn fly_past_node() {
        let mut current_position = Node::new(0, 0).as_vec3(0.0);
        let mut flight_plan = FlightPlan::new(vec![Node::new(2, 0), Node::new(1, 0)]);
        let mut travelled_route = TravelledRoute::new(vec![Node::new(0, 0)]);

        let movement_speed = (TILE_SIZE + (TILE_SIZE / 2)) as f32;

        fly(
            &mut current_position,
            &mut flight_plan,
            &mut travelled_route,
            movement_speed,
        );

        assert_eq!(current_position, Vec3::new(movement_speed, 0.0, 0.0));
        assert_eq!(1, flight_plan.get().len());
        assert_eq!(2, travelled_route.get().len());
    }

    #[test]
    fn fly_past_node_and_change_direction() {
        let mut current_position = Node::new(0, 0).as_vec3(0.0);
        let mut flight_plan = FlightPlan::new(vec![Node::new(1, 1), Node::new(1, 0)]);
        let mut travelled_route = TravelledRoute::new(vec![Node::new(0, 0)]);

        let movement_speed = (TILE_SIZE + (TILE_SIZE / 2)) as f32;

        fly(
            &mut current_position,
            &mut flight_plan,
            &mut travelled_route,
            movement_speed,
        );

        assert_eq!(current_position, Vec3::new(32.0, 16.0, 0.0));
        assert_eq!(1, flight_plan.get().len());
        assert_eq!(2, travelled_route.get().len());
    }
}
