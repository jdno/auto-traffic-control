use bevy::prelude::*;

use crate::components::AirplaneId;
use crate::map::Map;
use crate::{Event, EventBus};

pub fn despawn_airplane(
    mut commands: Commands,
    map: Res<Map>,
    query: Query<(Entity, &AirplaneId, &Transform)>,
    event_bus: Local<EventBus>,
) {
    let airport = map.airport().as_vec3(1.0);

    for (entity, airplane_id, transform) in query.iter() {
        if transform.translation == airport {
            commands.entity(entity).despawn_recursive();

            event_bus
                .sender()
                .send(Event::AirplaneLanded(airplane_id.clone()))
                .expect("failed to send event"); // TODO: Handle error
        }
    }
}
