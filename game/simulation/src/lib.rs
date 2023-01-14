//! Auto Traffic Control
//!
//! Auto Traffic Control is a video game played by programming. The player's task is to create a
//! program that can safely manage the airspace above two airports.
//!
//! This crate implements the game's simulation engine. The simulation has the following
//! responsibilities:
//!
//!   - Implement the game's entities and rules
//!   - Start and stop game sessions
//!
//! The simulation is not responsible for the API of the game, and can in fact be run without an API
//! altogether (e.g. for testing). It communicates with the API through command and event busses,
//! which are passed to the simulation as arguments.

use std::fmt::Display;

use crate::behavior::Observable;
use crate::bus::{Event, Sender};
use crate::state::State;

pub mod bus;
pub mod component;

mod behavior;
mod state;

const TILE_SIZE: u32 = 64;

#[derive(Clone, Debug)]
pub struct Simulation {
    event_bus: Sender<Event>,
    game: State,
}

impl Simulation {
    pub fn new(event_bus: Sender<Event>) -> Self {
        Self {
            event_bus: event_bus.clone(),
            game: State::new(event_bus),
        }
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.game)
    }
}

impl Observable for Simulation {
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
        let simulation = Simulation::new(sender);

        assert_eq!("game ready", simulation.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Simulation>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Simulation>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Simulation>();
    }
}
