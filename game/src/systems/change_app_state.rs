use bevy::prelude::*;

use crate::command::CommandBus;
use crate::{AppState, Command};

pub fn change_app_state(
    mut app_state: ResMut<State<AppState>>,
    mut command_bus: Local<CommandBus>,
) {
    let mut queued_transition = false;

    while let Ok(command) = command_bus.receiver().get_mut().try_recv() {
        match command {
            Command::StartGame => {
                app_state.set(AppState::Game).unwrap();
                queued_transition = true;
                break;
            }
            _ => continue,
        }
    }

    if queued_transition {
        // Drain remaining commands to prevent lagging
        while command_bus.receiver().get_mut().try_recv().is_ok() {}
    }
}
