use atc::v1::get_game_state_response::GameState;
use bevy::prelude::*;

use crate::{
    despawn_airplane, follow_flight_plan, setup_airport, setup_grid, spawn_airplane,
    update_flight_plan, SpawnTimer,
};

pub struct GameStateRunningPlugin;

impl Plugin for GameStateRunningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer::new(Timer::from_seconds(1.0, true)))
            .add_system_set(
                SystemSet::on_enter(GameState::Running)
                    .with_system(setup_airport)
                    .with_system(setup_grid),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(despawn_airplane)
                    .with_system(follow_flight_plan)
                    .with_system(spawn_airplane)
                    .with_system(update_flight_plan),
            )
            .add_system_set(SystemSet::on_exit(GameState::Running));
    }
}
