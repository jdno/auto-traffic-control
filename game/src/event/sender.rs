use bevy::prelude::*;
use tokio::sync::broadcast::Sender;

use crate::event::{Event, EventReceiver};

#[derive(Clone, Debug, Resource)]
pub struct EventSender(Sender<Event>);

impl EventSender {
    pub fn new(sender: Sender<Event>) -> Self {
        Self(sender)
    }

    pub fn get(&self) -> &Sender<Event> {
        &self.0
    }

    pub fn subscribe(&self) -> EventReceiver {
        EventReceiver::new(self.0.subscribe())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<EventSender>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<EventSender>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<EventSender>();
    }
}
