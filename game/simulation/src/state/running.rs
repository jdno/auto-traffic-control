use parking_lot::Mutex;
use std::fmt::Display;
use std::sync::Arc;
use time::Duration;

use crate::behavior::{Observable, Updateable};
use crate::bus::{Event, Sender};
use crate::entity::{Airplane, Spawner};
use crate::map::{Map, MapLoader, Maps};
use crate::state::Ready;

#[derive(Debug)]
#[allow(dead_code)] // TODO: Remove when map is used
pub struct Running {
    event_bus: Sender<Event>,

    airplanes: Vec<Airplane>,
    map: Arc<Mutex<Map>>,
    spawner: Spawner,
}

impl Running {
    pub fn new(event_bus: Sender<Event>) -> Self {
        let map = Arc::new(Mutex::new(MapLoader::load(Maps::Sandbox)));
        let spawner = Spawner::new(event_bus.clone(), map.clone(), Duration::seconds(2));

        let running = Self {
            event_bus,
            airplanes: Vec::new(),
            map: map.clone(),
            spawner,
        };

        // TODO: Handle error gracefully
        running
            .notify(Event::GameStarted(map.lock().clone()))
            .expect("failed to send GameStarted event");

        running
    }
}

impl Display for Running {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "running")
    }
}

impl From<&Ready> for Running {
    fn from(state: &Ready) -> Self {
        Self::new(state.event_bus().clone())
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
    use crate::bus::channel;

    use super::*;

    #[test]
    fn trait_display() {
        let (sender, _receiver) = channel(256);
        let running = Running::new(sender);

        assert_eq!("running", running.to_string());
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Running>();
    }
}
