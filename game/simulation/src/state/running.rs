use std::fmt::Display;
use std::sync::Arc;

use hecs::World;
use parking_lot::Mutex;
use time::Duration;

use crate::behavior::{Commandable, Observable, Updateable};
use crate::bus::{Command, Event, Receiver, Sender, COMMAND_BUS, EVENT_BUS};
use crate::map::{MapLoader, Maps};
use crate::state::Ready;
use crate::system::{
    DespawnAirplaneSystem, DetectCollisionSystem, GenerateFlightPlanSystem, MoveAirplaneSystem,
    SpawnAirplaneSystem, System, UpdateFlightPlanSystem,
};

pub struct Running {
    command_bus: Receiver<Command>,
    event_bus: Sender<Event>,

    systems: Vec<Box<dyn System>>,
    world: World,
}

impl Running {
    pub fn new() -> Self {
        let map = MapLoader::load(Maps::Sandbox);
        let airports = map.airports().clone();
        let grid = map.grid().clone();
        let width = map.width();
        let height = map.height();

        let map = Arc::new(Mutex::new(map));

        let systems: Vec<Box<dyn System>> = vec![
            Box::new(DespawnAirplaneSystem::new(EVENT_BUS.0.clone(), map.clone())),
            Box::new(UpdateFlightPlanSystem::new(
                COMMAND_BUS.1.resubscribe(),
                EVENT_BUS.0.clone(),
                map.clone(),
            )),
            Box::new(GenerateFlightPlanSystem::new(
                EVENT_BUS.0.clone(),
                map.clone(),
            )),
            Box::new(MoveAirplaneSystem::new(EVENT_BUS.0.clone())),
            Box::new(DetectCollisionSystem::new(EVENT_BUS.0.clone())),
            Box::new(SpawnAirplaneSystem::new(
                EVENT_BUS.0.clone(),
                map,
                Duration::seconds(2),
            )),
        ];

        let running = Self {
            command_bus: COMMAND_BUS.1.resubscribe(),
            event_bus: EVENT_BUS.0.clone(),
            systems,
            world: World::new(),
        };

        // TODO: Handle error gracefully
        running
            .notify(Event::GameStarted(airports, grid, width, height))
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

impl Drop for Running {
    fn drop(&mut self) {
        self.event_bus
            .send(Event::GameStopped)
            .expect("failed to send GameStopped event");
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
        for system in &mut self.systems {
            system.update(&mut self.world, delta);
        }
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
