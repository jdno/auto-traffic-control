use std::fmt::Display;

use crate::behavior::{Commandable, Observable, Updateable};
use crate::bus::{Command, Event, Receiver, Sender, COMMAND_BUS, EVENT_BUS};
use crate::state::Running;

#[derive(Debug)]
pub struct Ready {
    command_bus: Receiver<Command>,
    event_bus: Sender<Event>,
}

impl Ready {
    pub fn new() -> Self {
        Self {
            command_bus: COMMAND_BUS.1.resubscribe(),
            event_bus: EVENT_BUS.0.clone(),
        }
    }
}

impl Commandable for Ready {
    fn command_bus(&self) -> &Receiver<Command> {
        &self.command_bus
    }
}

impl Display for Ready {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ready")
    }
}

impl Observable for Ready {
    fn event_bus(&self) -> &Sender<Event> {
        &self.event_bus
    }
}

impl From<&Running> for Ready {
    fn from(_state: &Running) -> Self {
        Ready::new()
    }
}

impl Updateable for Ready {
    fn update(&mut self, _delta: f32) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let ready = Ready::new();

        assert_eq!("ready", ready.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Ready>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Ready>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Ready>();
    }
}
