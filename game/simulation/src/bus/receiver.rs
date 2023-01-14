use std::fmt::{Display, Formatter};

use crate::bus::{RecvError, TryRecvError};

#[derive(Debug)]
pub struct Receiver<T> {
    pub(super) receiver: tokio::sync::broadcast::Receiver<T>,
}

impl<T> Receiver<T>
where
    T: Clone,
{
    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }

    pub async fn recv(&mut self) -> Result<T, RecvError> {
        self.receiver.recv().await
    }

    pub fn resubscribe(&self) -> Self {
        Self {
            receiver: self.receiver.resubscribe(),
        }
    }
}

impl<T> Display for Receiver<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Receiver")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Receiver<()>>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Receiver<()>>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Receiver<()>>();
    }
}
