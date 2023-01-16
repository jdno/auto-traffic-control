use hecs::World;

use crate::bus::{Event, Sender};
use crate::component::{AirplaneId, FlightPlan, TravelledRoute};
use crate::map::Location;
use crate::system::System;
use crate::TILE_SIZE;

const AIRPLANE_SPEED: f32 = TILE_SIZE as f32;

#[derive(Clone, Debug)]
pub struct MoveAirplaneSystem {
    event_bus: Sender<Event>,
}

impl MoveAirplaneSystem {
    pub fn new(event_bus: Sender<Event>) -> Self {
        Self { event_bus }
    }
}

impl System for MoveAirplaneSystem {
    fn update(&mut self, world: &mut World, delta: f32) {
        for (_entity, (airplane_id, location, flight_plan, travelled_route)) in world.query_mut::<(
            &AirplaneId,
            &mut Location,
            &mut FlightPlan,
            &mut TravelledRoute,
        )>() {
            let mut distance = AIRPLANE_SPEED * delta;

            while distance > 0.0 {
                let next_node = flight_plan
                    .get()
                    .iter()
                    .last()
                    .expect("flight plan must not be empty");
                let next_location = next_node.into();

                let distance_to_next_location = location.euclidean_distance(&next_location) as f32;

                if distance >= distance_to_next_location {
                    *location = next_location;

                    let node = flight_plan.get_mut().pop().unwrap();
                    travelled_route.get_mut().push(node);

                    distance -= distance_to_next_location;

                    self.event_bus
                        .send(Event::FlightPlanUpdated(
                            airplane_id.clone(),
                            flight_plan.clone(),
                        ))
                        .expect("failed to send FlightPlanUpdated event");
                } else {
                    *location = location
                        .move_towards(&next_location, distance as f64)
                        .expect("failed to move towards next location");

                    distance = 0.0;

                    self.event_bus
                        .send(Event::AirplaneMoved(airplane_id.clone(), *location))
                        .expect("failed to send AirplaneMoved event");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::bus::channel;
    use crate::map::Node;

    use super::*;

    #[test]
    fn move_airplane() {
        let (sender, mut receiver) = channel(2);
        let mut system = MoveAirplaneSystem::new(sender);

        let mut world = World::new();
        world.spawn((
            AirplaneId::default(),
            Location::new(32.0, 0.0),
            FlightPlan::new(vec![
                Arc::new(Node::new(2, 0, false)),
                Arc::new(Node::new(1, 0, false)),
            ]),
            TravelledRoute::new(vec![]),
        ));

        system.update(&mut world, 1.0);

        let event = receiver.try_recv().unwrap();
        assert_eq!(
            Event::FlightPlanUpdated(
                AirplaneId::default(),
                FlightPlan::new(vec![Arc::new(Node::new(2, 0, false))])
            ),
            event
        );

        let event = receiver.try_recv().unwrap();
        assert_eq!(
            Event::AirplaneMoved(AirplaneId::default(), Location::new(96.0, 0.0)),
            event
        );
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<MoveAirplaneSystem>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<MoveAirplaneSystem>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<MoveAirplaneSystem>();
    }
}
