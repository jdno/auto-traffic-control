use bevy::prelude::*;
use tokio::sync::broadcast::Receiver;

use crate::event::Event;

#[derive(Debug, Resource)]
pub struct EventReceiver(Receiver<Event>);

impl EventReceiver {
    pub fn new(receiver: Receiver<Event>) -> Self {
        Self(receiver)
    }

    pub fn get_mut(&mut self) -> &mut Receiver<Event> {
        &mut self.0
    }
}

impl From<EventReceiver> for Receiver<Event> {
    fn from(value: EventReceiver) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<EventReceiver>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<EventReceiver>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<EventReceiver>();
    }
}
