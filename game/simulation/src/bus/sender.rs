use std::fmt::{Display, Formatter};

use crate::bus::{Receiver, SendError};

#[derive(Clone, Debug)]
pub struct Sender<T> {
    pub(super) sender: tokio::sync::broadcast::Sender<T>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<usize, SendError<T>> {
        self.sender.send(value)
    }

    pub fn subscribe(&self) -> Receiver<T> {
        Receiver {
            receiver: self.sender.subscribe(),
        }
    }
}

impl<T> Display for Sender<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sender")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Sender<()>>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Sender<()>>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Sender<()>>();
    }
}
