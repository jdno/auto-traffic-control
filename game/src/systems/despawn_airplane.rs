use bevy::prelude::*;

use crate::components::{AirplaneId, Landing};
use crate::event::{Event, EventBus};
use crate::map::Map;
use crate::resources::Score;

pub fn despawn_airplane(
    mut commands: Commands,
    map: Res<Map>,
    mut score: ResMut<Score>,
    query: Query<(Entity, &AirplaneId, &Transform), With<Landing>>,
    event_bus: Local<EventBus>,
) {
    let airport = map.airport().node().as_vec3(2.0);

    for (entity, airplane_id, transform) in query.iter() {
        if transform.translation == airport {
            commands.entity(entity).despawn_recursive();

            score.increment();

            event_bus
                .sender()
                .send(Event::AirplaneLanded(airplane_id.clone()))
                .expect("failed to send event"); // TODO: Handle error
        }
    }
}
