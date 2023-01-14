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

use crate::behavior::{Observable, Updateable};
use crate::bus::{Command, Event, Receiver, Sender};
use crate::state::State;

pub mod behavior;
pub mod bus;

mod component;
mod entity;
mod map;
mod state;

const TILE_SIZE: u32 = 64;

#[derive(Debug)]
pub struct Simulation {
    command_bus: Receiver<Command>,
    event_bus: Sender<Event>,
    game: State,
}

impl Simulation {
    pub fn new(command_bus: Receiver<Command>, event_bus: Sender<Event>) -> Self {
        Self {
            command_bus,
            event_bus: event_bus.clone(),
            game: State::new(event_bus),
        }
    }

    fn start_game(&mut self) {
        if let State::Ready(ready) = &self.game {
            self.game = State::Running(ready.into());
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

impl Updateable for Simulation {
    fn update(&mut self, _delta: f32) {
        while let Ok(command) = self.command_bus.try_recv() {
            match command {
                Command::StartGame => self.start_game(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bus::channel;

    use super::*;

    #[test]
    fn trait_display() {
        let (_, command_receiver) = channel(1);
        let (event_sender, _) = channel(1);

        let simulation = Simulation::new(command_receiver, event_sender);

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
