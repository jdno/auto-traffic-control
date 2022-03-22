use bevy::prelude::*;

use atc::v1::get_game_state_response::GameState;

use crate::event::{Event, EventBus};
use crate::map::Map;
use crate::resources::Score;
use crate::systems::{
    despawn_airplane, detect_collision, follow_flight_plan, generate_flight_plan, land_airplane,
    setup_airport, setup_grid, spawn_airplane, update_flight_plan, SpawnTimer,
};

pub struct GameStateRunningPlugin;

impl Plugin for GameStateRunningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer::new(Timer::from_seconds(2.0, true)))
            .insert_resource(Map::new())
            .add_system_set(
                SystemSet::on_enter(GameState::Running)
                    .with_system(start_game)
                    .with_system(setup_airport)
                    .with_system(setup_grid),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(follow_flight_plan.label("move"))
                    .with_system(despawn_airplane)
                    .with_system(detect_collision)
                    .with_system(generate_flight_plan)
                    .with_system(land_airplane)
                    .with_system(spawn_airplane)
                    .with_system(update_flight_plan),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Running)
                    .with_system(despawn_entities)
                    .with_system(end_game),
            );
    }
}

fn start_game(mut commands: Commands, map: Res<Map>, event_bus: Local<EventBus>) {
    commands.insert_resource(Score::new());

    let map = map.into_inner().clone();

    event_bus
        .sender()
        .send(Event::GameStarted(map))
        .expect("failed to send event"); // TODO: Handle error
}

fn end_game(score: Res<Score>, event_bus: Local<EventBus>) {
    event_bus
        .sender()
        .send(Event::GameStopped(*score))
        .expect("failed to send event"); // TODO: Handle error
}

fn despawn_entities(mut commands: Commands, query: Query<Entity, Without<Camera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
