use crate::bus::{Command, Receiver};

pub trait Commandable {
    fn command_bus(&self) -> &Receiver<Command>;
}
