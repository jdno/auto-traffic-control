use crate::behavior::Updateable;
use std::fmt::{Display, Formatter};

use crate::bus::{Event, Sender};

pub use self::ready::*;
pub use self::running::*;

mod ready;
mod running;

#[derive(Debug)]
pub enum State {
    Ready(Ready),
    Running(Running),
}

impl State {
    pub fn new(event_bus: Sender<Event>) -> Self {
        Self::Ready(Ready::new(event_bus))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let payload = match self {
            Self::Ready(state) => state.to_string(),
            Self::Running(state) => state.to_string(),
        };

        write!(f, "game {}", payload)
    }
}

impl Updateable for State {
    fn update(&mut self, delta: f32) {
        match self {
            Self::Ready(state) => state.update(delta),
            Self::Running(state) => state.update(delta),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let (sender, _) = crate::bus::channel(256);
        let game = State::new(sender);

        assert_eq!("game ready", game.to_string());
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<State>();
    }
}
