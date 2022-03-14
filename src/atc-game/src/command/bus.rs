use bevy::prelude::*;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::command::Command;

pub type CommandReceiver = Receiver<Command>;
pub type CommandSender = Sender<Command>;

#[derive(Clone, Debug)]
pub struct CommandBus {
    sender: CommandSender,
}

impl CommandBus {
    pub fn sender(&self) -> &CommandSender {
        &self.sender
    }
}

impl FromWorld for CommandBus {
    fn from_world(world: &mut World) -> Self {
        let sender = world.get_resource::<CommandSender>().unwrap();

        Self {
            sender: sender.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommandBus;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CommandBus>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CommandBus>();
    }
}
