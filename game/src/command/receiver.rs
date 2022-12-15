use bevy::prelude::*;
use tokio::sync::broadcast::Receiver;

use crate::command::Command;

#[derive(Debug, Resource)]
pub struct CommandReceiver(Receiver<Command>);

impl CommandReceiver {
    pub fn new(receiver: Receiver<Command>) -> Self {
        Self(receiver)
    }

    pub fn get_mut(&mut self) -> &mut Receiver<Command> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CommandReceiver>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CommandReceiver>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<CommandReceiver>();
    }
}
