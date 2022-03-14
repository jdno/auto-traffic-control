use bevy::prelude::*;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::command::Command;

pub type CommandReceiver = Receiver<Command>;
pub type CommandSender = Sender<Command>;

#[derive(Debug)]
pub struct CommandBus {
    receiver: CommandReceiver,
}

impl CommandBus {
    pub fn receiver(&mut self) -> &mut CommandReceiver {
        &mut self.receiver
    }
}

impl FromWorld for CommandBus {
    fn from_world(world: &mut World) -> Self {
        let sender = world.get_resource::<CommandSender>().unwrap();

        Self {
            receiver: sender.subscribe(),
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
