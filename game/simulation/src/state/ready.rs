use crate::behavior::{Observable, Updateable};
use std::fmt::Display;

use crate::bus::{Event, Sender};
use crate::state::Running;

#[derive(Clone, Debug)]
pub struct Ready {
    event_bus: Sender<Event>,
}

impl Ready {
    pub fn new(event_bus: Sender<Event>) -> Self {
        Self { event_bus }
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
    fn from(state: &Running) -> Self {
        Ready::new(state.event_bus().clone())
    }
}

impl Updateable for Ready {
    fn update(&mut self, _delta: f32) {}
}

#[cfg(test)]
mod tests {
    use crate::bus::channel;

    use super::*;

    #[test]
    fn trait_display() {
        let (sender, _) = channel(256);
        let ready = Ready::new(sender);

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
