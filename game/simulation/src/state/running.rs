use std::fmt::Display;
use std::sync::Arc;

use parking_lot::Mutex;
use time::Duration;

use crate::behavior::{Commandable, Observable, Updateable};
use crate::bus::{Command, Event, Receiver, Sender, COMMAND_BUS, EVENT_BUS};
use crate::entity::{Airplane, Spawner};
use crate::map::{Map, MapLoader, Maps};
use crate::state::Ready;

#[derive(Debug)]
#[allow(dead_code)] // TODO: Remove when map is used
pub struct Running {
    command_bus: Receiver<Command>,
    event_bus: Sender<Event>,

    airplanes: Vec<Airplane>,
    map: Arc<Mutex<Map>>,
    spawner: Spawner,
}

impl Running {
    pub fn new() -> Self {
        let map = Arc::new(Mutex::new(MapLoader::load(Maps::Sandbox)));
        let spawner = Spawner::new(map.clone(), Duration::seconds(2));

        let running = Self {
            command_bus: COMMAND_BUS.1.resubscribe(),
            event_bus: EVENT_BUS.0.clone(),
            airplanes: Vec::new(),
            map: map.clone(),
            spawner,
        };

        let map = map.lock();

        // TODO: Handle error gracefully
        running
            .notify(Event::GameStarted(
                map.airports().clone(),
                map.grid().clone(),
                map.width(),
                map.height(),
            ))
            .expect("failed to send GameStarted event");

        running
    }
}

impl Commandable for Running {
    fn command_bus(&self) -> &Receiver<Command> {
        &self.command_bus
    }
}

impl Display for Running {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "running")
    }
}

impl From<&Ready> for Running {
    fn from(_state: &Ready) -> Self {
        Self::new()
    }
}

impl Observable for Running {
    fn event_bus(&self) -> &Sender<Event> {
        &self.event_bus
    }
}

impl Updateable for Running {
    fn update(&mut self, delta: f32) {
        self.spawner.update(delta);
        self.airplanes
            .iter_mut()
            .for_each(|airplane| airplane.update(delta));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let running = Running::new();

        assert_eq!("running", running.to_string());
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Running>();
    }
}
