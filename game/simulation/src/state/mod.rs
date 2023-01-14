use std::fmt::{Display, Formatter};

use crate::behavior::Updateable;

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
    pub fn new() -> Self {
        Self::Ready(Ready::new())
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
        let game = State::new();

        assert_eq!("game ready", game.to_string());
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<State>();
    }
}
