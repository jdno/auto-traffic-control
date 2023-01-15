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

use crate::behavior::{Commandable, Updateable};
use crate::bus::{Command, Event, Receiver, Sender, COMMAND_BUS, EVENT_BUS};
use crate::state::{Ready, State};

pub mod behavior;
pub mod bus;

mod component;
mod map;
mod state;
mod system;
mod util;

const TILE_SIZE: u32 = 64;

pub struct Simulation {
    command_bus: Receiver<Command>,
    event_bus: Receiver<Event>,
    game: State,
}

impl Simulation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn command_bus(&self) -> Sender<Command> {
        COMMAND_BUS.0.clone()
    }

    pub fn event_bus(&self) -> Receiver<Event> {
        EVENT_BUS.1.resubscribe()
    }

    fn start_game(&mut self) {
        if let State::Ready(ready) = &self.game {
            self.game = State::Running(ready.into());
        }
    }
}

impl Commandable for Simulation {
    fn command_bus(&self) -> &Receiver<Command> {
        &self.command_bus
    }
}

impl Default for Simulation {
    fn default() -> Self {
        Self {
            command_bus: COMMAND_BUS.1.resubscribe(),
            event_bus: EVENT_BUS.1.resubscribe(),
            game: State::new(),
        }
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.game)
    }
}

impl Updateable for Simulation {
    fn update(&mut self, delta: f32) {
        while let Ok(command) = self.command_bus.try_recv() {
            match command {
                Command::StartGame => self.start_game(),
                _ => continue,
            }
        }

        self.game.update(delta);

        while let Ok(event) = self.event_bus.try_recv() {
            if let Event::AirplaneCollided(_, _) = event {
                self.game = State::Ready(Ready::new());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let simulation = Simulation::new();

        assert_eq!("game ready", simulation.to_string());
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Simulation>();
    }
}
