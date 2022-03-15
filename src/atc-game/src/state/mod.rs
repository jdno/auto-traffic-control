use crate::event::EventReceiver;
use crate::{Event, SharedGameState};
use atc::v1::get_game_state_response::GameState;

pub use self::ready::GameStateReadyPlugin;
pub use self::running::GameStateRunningPlugin;

mod ready;
mod running;

#[derive(Debug)]
pub struct GameStateWatcher {
    event_bus: EventReceiver,
    game_state: SharedGameState,
}

impl GameStateWatcher {
    pub fn new(event_bus: EventReceiver, game_state: SharedGameState) -> Self {
        Self {
            event_bus,
            game_state,
        }
    }

    pub async fn connect(&mut self) {
        while let Ok(event) = self.event_bus.recv().await {
            match event {
                Event::GameStarted => {
                    let mut game_started = self.game_state.lock();
                    *game_started = GameState::Running;
                }
                Event::GameStopped => {
                    let mut game_started = self.game_state.lock();
                    *game_started = GameState::Ready;
                }
                _ => {}
            }
        }
    }
}
