use bevy::prelude::*;

use crate::components::{AirplaneId, FlightPlan, Landing, Tag};
use crate::event::{Event, EventBus};
use crate::map::{Airport, Map, Node};
use crate::resources::Score;

pub fn despawn_airplane(
    mut commands: Commands,
    map: Res<Map>,
    mut score: ResMut<Score>,
    mut query: Query<(Entity, &AirplaneId, &mut FlightPlan, &Tag, &Transform), With<Landing>>,
    event_bus: Local<EventBus>,
) {
    let airport_location = map.airport().node().as_vec3(2.0);
    let airport_tag = map.airport().tag();

    for (entity, airplane_id, mut flight_plan, tag, transform) in query.iter_mut() {
        let airplane_location = transform.translation;

        if airplane_location != airport_location {
            continue;
        }

        if tag == &airport_tag {
            commands.entity(entity).despawn_recursive();

            score.increment();

            event_bus
                .sender()
                .send(Event::AirplaneLanded(airplane_id.clone()))
                .expect("failed to send event"); // TODO: Handle error
        } else {
            let go_around = go_around_procedure(map.airport());

            *flight_plan = FlightPlan::new(vec![go_around]);

            commands.entity(entity).remove::<Landing>();

            event_bus
                .sender()
                .send(Event::LandingAborted(airplane_id.clone()))
                .expect("failed to send event"); // TODO: Handle error

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

fn go_around_procedure(airport: &Airport) -> Node {
    let runway_direction = airport.runway().to_vec3();
    let next_hop_in_direction = runway_direction * Vec3::splat(-2.0);

    Node::unrestricted(
        next_hop_in_direction.x as i32,
        next_hop_in_direction.y as i32,
    )
}
