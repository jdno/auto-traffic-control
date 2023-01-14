use std::fmt::Display;

use crate::behavior::Observable;
use crate::bus::{Event, Sender};

#[derive(Clone, Debug)]
pub struct Running {
    event_bus: Sender<Event>,
}

impl Running {
    #[allow(dead_code)] // TODO: Remove when changing states is implemented
    pub fn new(event_bus: Sender<Event>) -> Self {
        Self { event_bus }
    }
}

impl Display for Running {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "running")
    }
}

impl Observable for Running {
    fn event_bus(&self) -> &Sender<Event> {
        &self.event_bus
    }
}

#[cfg(test)]
mod tests {
    use crate::bus::channel;

    use super::*;

    #[test]
    fn trait_display() {
        let (sender, _) = channel(256);
        let running = Running::new(sender);

        assert_eq!("running", running.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Running>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Running>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Running>();
    }
}
