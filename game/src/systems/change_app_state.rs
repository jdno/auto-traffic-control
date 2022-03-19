use bevy::prelude::*;

use crate::Command;
use atc::v1::get_game_state_response::GameState;

use crate::command::CommandBus;

pub fn change_app_state(
    mut app_state: ResMut<State<GameState>>,
    mut command_bus: Local<CommandBus>,
) {
    let mut queued_transition = false;

    while let Ok(command) = command_bus.receiver().try_recv() {
        match command {
            Command::StartGame => {
                app_state.set(GameState::Running).unwrap();
                queued_transition = true;
                break;
            }
            _ => continue,
        }
    }

    if queued_transition {
        // Drain remaining commands to prevent lagging
        while command_bus.receiver().try_recv().is_ok() {}
    }
}
