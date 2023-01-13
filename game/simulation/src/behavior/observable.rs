use crate::bus::{Event, SendError, Sender};

pub trait Observable {
    fn event_bus(&self) -> &Sender<Event>;

    fn notify(&self, event: Event) -> Result<usize, SendError<Event>> {
        self.event_bus().send(event)
    }
}
