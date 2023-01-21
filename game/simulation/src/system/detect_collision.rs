use hecs::World;

use crate::bus::{Event, Sender};
use crate::component::AirplaneId;
use crate::map::Location;
use crate::system::System;
use crate::TILE_SIZE;

const THRESHOLD: f32 = TILE_SIZE as f32 * 0.75;

#[derive(Clone, Debug)]
pub struct DetectCollisionSystem {
    event_bus: Sender<Event>,
}

impl DetectCollisionSystem {
    pub fn new(event_bus: Sender<Event>) -> Self {
        Self { event_bus }
    }
}

impl System for DetectCollisionSystem {
    fn update(&mut self, world: &mut World, _delta: f32) {
        'outer: for (_entity, (airplane_id, location)) in
            world.query::<(&AirplaneId, &Location)>().iter()
        {
            for (_other_entity, (other_airplane_id, other_location)) in
                world.query::<(&AirplaneId, &Location)>().iter()
            {
                if airplane_id == other_airplane_id {
                    continue;
                }

                let distance = location.euclidean_distance(other_location) as f32;

                if distance <= THRESHOLD {
                    self.event_bus
                        .send(Event::AirplaneCollided(
                            airplane_id.clone(),
                            other_airplane_id.clone(),
                        ))
                        .expect("failed to send AirplaneCollided event");

                    break 'outer;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::bus::channel;
    use crate::component::{FlightPlan, Tag};
    use crate::map::Node;

    use super::*;

    #[test]
    fn no_collision() {
        let (sender, mut receiver) = channel(1);
        let mut world = World::new();

        let mut system = DetectCollisionSystem::new(sender);

        world.spawn((
            AirplaneId::default(),
            Location::new(0.0, 0.0),
            FlightPlan::new(vec![Arc::new(Node::new(0, 0, false))]),
            Tag::Blue,
        ));

        world.spawn((
            AirplaneId::default(),
            Location::new(64.0, 0.0),
            FlightPlan::new(vec![Arc::new(Node::new(0, 0, false))]),
            Tag::Blue,
        ));

        system.update(&mut world, 0.0);

        assert!(receiver.try_recv().is_err());
    }

    #[test]
    fn collision() {
        let (sender, mut receiver) = channel(10);
        let mut world = World::new();

        let mut system = DetectCollisionSystem::new(sender);

        world.spawn((
            AirplaneId::new("AT-0001".into()),
            Location::new(0.0, 0.0),
            FlightPlan::new(vec![Arc::new(Node::new(0, 0, false))]),
            Tag::Blue,
        ));

        world.spawn((
            AirplaneId::new("AT-0002".into()),
            Location::new(64.0, 0.0),
            FlightPlan::new(vec![Arc::new(Node::new(1, 0, false))]),
            Tag::Blue,
        ));

        world.spawn((
            AirplaneId::new("AT-0003".into()),
            Location::new(16.0, 0.0),
            FlightPlan::new(vec![Arc::new(Node::new(0, 0, false))]),
            Tag::Blue,
        ));

        // Collision with this plane will never the checked, because the game has already ended.
        world.spawn((
            AirplaneId::new("AT-0004".into()),
            Location::new(0.0, 16.0),
            FlightPlan::new(vec![Arc::new(Node::new(0, 0, false))]),
            Tag::Blue,
        ));

        system.update(&mut world, 0.0);

        let event = receiver.try_recv().unwrap();

        assert_eq!(
            Event::AirplaneCollided(
                AirplaneId::new("AT-0001".into()),
                AirplaneId::new("AT-0003".into())
            ),
            event
        );

        assert!(receiver.try_recv().is_err());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<DetectCollisionSystem>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<DetectCollisionSystem>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<DetectCollisionSystem>();
    }
}
