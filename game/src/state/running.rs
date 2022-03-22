use bevy::prelude::*;

use atc::v1::get_game_state_response::GameState;

use crate::event::{Event, EventBus};
use crate::map::Map;
use crate::systems::{
    despawn_airplane, detect_collision, follow_flight_plan, generate_flight_plan, setup_airport,
    setup_grid, spawn_airplane, update_flight_plan, SpawnTimer,
};

pub struct GameStateRunningPlugin;

impl Plugin for GameStateRunningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer::new(Timer::from_seconds(1.0, true)))
            .insert_resource(Map::new())
            .add_system_set(
                SystemSet::on_enter(GameState::Running)
                    .with_system(send_event)
                    .with_system(setup_airport)
                    .with_system(setup_grid),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(follow_flight_plan.label("move"))
                    .with_system(despawn_airplane.label("despawn").after("move"))
                    .with_system(generate_flight_plan.after("despawn"))
                    .with_system(detect_collision.after("despawn"))
                    .with_system(spawn_airplane)
                    .with_system(update_flight_plan),
            )
            .add_system_set(SystemSet::on_exit(GameState::Running).with_system(despawn_entities));
    }
}

fn send_event(map: Res<Map>, event_bus: Local<EventBus>) {
    let map = map.into_inner().clone();

    event_bus
        .sender()
        .send(Event::GameStarted(map))
        .expect("failed to send event"); // TODO: Handle error
}

fn despawn_entities(mut commands: Commands, query: Query<Entity, Without<Camera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
