use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use atc::v1::get_game_state_response::GameState;

use crate::components::{AirplaneId, Collider, AIRPLANE_SIZE};
use crate::event::{Event, EventBus};

pub struct Size {
    airplane: Vec2,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            airplane: Vec2::new(AIRPLANE_SIZE, AIRPLANE_SIZE),
        }
    }
}

pub fn detect_collision(
    mut app_state: ResMut<State<GameState>>,
    query: Query<(Entity, &AirplaneId, &Collider, &Transform)>,
    event_bus: Local<EventBus>,
    size: Local<Size>,
) {
    'outer: for (entity1, airplane_id1, _, transform1) in query.iter() {
        for (entity2, airplane_id2, _, transform2) in query.iter() {
            if entity1 == entity2 {
                continue;
            }

            if collide(
                transform1.translation,
                size.airplane,
                transform2.translation,
                size.airplane,
            )
            .is_some()
            {
                event_bus
                    .sender()
                    .send(Event::AirplaneCollided(
                        airplane_id1.clone(),
                        airplane_id2.clone(),
                    ))
                    .expect("failed to send event"); // TODO: Handle error

                app_state.set(GameState::Ready).unwrap();

                break 'outer;
            }
        }
    }
}
