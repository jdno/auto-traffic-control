use tokio::sync::broadcast;
pub use tokio::sync::broadcast::error::{RecvError, SendError, TryRecvError};

use crate::bus::receiver::Receiver;
use crate::bus::sender::Sender;

pub fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>)
where
    T: Clone,
{
    let (sender, receiver) = broadcast::channel(buffer);
    (Sender { sender }, Receiver { receiver })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn one_sender_to_two_receivers() {
        let (tx, mut rx1) = channel::<usize>(16);
        let mut rx2 = tx.subscribe();

        tokio::spawn(async move {
            assert_eq!(rx1.recv().await.unwrap(), 10);
            assert_eq!(rx1.recv().await.unwrap(), 20);
        });

        tokio::spawn(async move {
            assert_eq!(rx2.recv().await.unwrap(), 10);
            assert_eq!(rx2.recv().await.unwrap(), 20);
        });

        tx.send(10).unwrap();
        tx.send(20).unwrap();
    }
}
