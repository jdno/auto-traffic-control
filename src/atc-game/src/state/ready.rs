use bevy::prelude::*;

use atc::v1::game_state_response::GameState;

pub struct GameStateReadyPlugin;

impl Plugin for GameStateReadyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Ready))
            .add_system_set(SystemSet::on_update(GameState::Ready))
            .add_system_set(SystemSet::on_exit(GameState::Ready));
    }
}
