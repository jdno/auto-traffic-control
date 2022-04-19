use bevy::prelude::*;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::event::Event;

pub type EventReceiver = Receiver<Event>;
pub type EventSender = Sender<Event>;

#[derive(Clone, Debug)]
pub struct EventBus {
    sender: EventSender,
}

impl EventBus {
    pub fn sender(&self) -> &EventSender {
        &self.sender
    }
}

impl FromWorld for EventBus {
    fn from_world(world: &mut World) -> Self {
        let sender = world.resource::<EventSender>();

        Self {
            sender: sender.clone(),
        }
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
