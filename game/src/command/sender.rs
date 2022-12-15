use bevy::prelude::*;
use tokio::sync::broadcast::Sender;

use crate::command::{Command, CommandReceiver};

#[derive(Clone, Debug, Resource)]
pub struct CommandSender(Sender<Command>);

impl CommandSender {
    pub fn new(sender: Sender<Command>) -> Self {
        Self(sender)
    }

    pub fn get(&self) -> &Sender<Command> {
        &self.0
    }

    pub fn subscribe(&self) -> CommandReceiver {
        CommandReceiver::new(self.0.subscribe())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CommandSender>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CommandSender>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<CommandSender>();
    }
}
