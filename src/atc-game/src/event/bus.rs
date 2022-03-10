use tokio::sync::broadcast::Sender;

use crate::event::Event;

pub type EventSender = Sender<Event>;

#[derive(Clone, Debug)]
pub struct EventBus {
    sender: EventSender,
}

impl EventBus {
    pub fn new(sender: EventSender) -> Self {
        Self { sender }
    }

    pub fn sender(&self) -> &EventSender {
        &self.sender
    }
}

#[cfg(test)]
mod tests {
    use super::EventBus;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<EventBus>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<EventBus>();
    }
}
