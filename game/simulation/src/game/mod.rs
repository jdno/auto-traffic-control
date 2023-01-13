use std::fmt::{Display, Formatter};

use crate::behavior::Observable;
use crate::bus::{Event, Sender};

pub use self::ready::*;
pub use self::running::*;

mod ready;
mod running;

#[derive(Clone, Debug)]
pub struct Game<S> {
    event_bus: Sender<Event>,
    state: S,
}

impl Game<Ready> {
    pub fn new(event_bus: Sender<Event>) -> Self {
        Self {
            event_bus,
            state: Ready::new(),
        }
    }
}

impl<S> Display for Game<S>
where
    S: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "game {}", self.state)
    }
}

impl From<Game<Ready>> for Game<Running> {
    fn from(game: Game<Ready>) -> Self {
        game.notify(Event::GameStarted)
            .expect("failed to send game started event");

        Self {
            event_bus: game.event_bus,
            state: Running::new(),
        }
    }
}

impl From<Game<Running>> for Game<Ready> {
    fn from(game: Game<Running>) -> Self {
        game.notify(Event::GameStopped)
            .expect("failed to send game stopped event");

        Self {
            event_bus: game.event_bus,
            state: Ready::new(),
        }
    }
}

impl<S> Observable for Game<S> {
    fn event_bus(&self) -> &Sender<Event> {
        &self.event_bus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let (sender, _) = crate::bus::channel(256);
        let game = Game::new(sender);

        assert_eq!("game ready", game.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Game<Ready>>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Game<Ready>>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Game<Ready>>();
    }
}
