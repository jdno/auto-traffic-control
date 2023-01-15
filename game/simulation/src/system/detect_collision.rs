use crate::bus::{Event, Sender};
use crate::component::AirplaneId;
use crate::map::Location;
use crate::system::System;
use hecs::World;

const THRESHOLD: f32 = 24.0;

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
                }

                break 'outer;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
