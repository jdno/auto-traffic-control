use bevy::prelude::*;

use crate::command::{CommandReceiver, CommandSender};

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
        let sender = world.resource::<CommandSender>();

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
