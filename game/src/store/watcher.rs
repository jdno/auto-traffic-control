use std::sync::Arc;

use atc::v1::get_game_state_response::GameState;
use atc::v1::Airplane;

use crate::api::AsApi;
use crate::components::{AirplaneId, FlightPlan, Location};
use crate::event::EventReceiver;
use crate::map::Map;
use crate::{Event, Store};

#[derive(Debug)]
pub struct StoreWatcher {
    event_bus: EventReceiver,
    store: Arc<Store>,
}

impl StoreWatcher {
    pub fn new(event_bus: EventReceiver, store: Arc<Store>) -> Self {
        Self { event_bus, store }
    }

    pub async fn connect(&mut self) {
        while let Ok(event) = self.event_bus.recv().await {
            match event {
                Event::AirplaneDetected(id, location, flight_plan) => {
                    self.insert_airplane(id, location, flight_plan)
                }
                Event::AirplaneLanded(id) => self.remove_airplane(id),
                Event::AirplaneMoved(id, location) => self.move_airplane(id, location),
                Event::FlightPlanUpdated(id, flight_plan) => {
                    self.update_flight_plan(id, flight_plan)
                }
                Event::GameStarted(map) => self.start_game(map),
                Event::GameStopped => self.reset(),
                _ => {}
            }
        }
    }

    fn insert_airplane(&self, id: AirplaneId, location: Location, flight_plan: FlightPlan) {
        self.store.airplanes().insert(
            id.get().into(),
            Airplane {
                id: id.as_api(),
                point: Some(location.as_api()),
                flight_plan: flight_plan.as_api(),
            },
        );
    }

    fn remove_airplane(&self, id: AirplaneId) {
        self.store.airplanes().remove(id.get());
    }

    fn move_airplane(&self, id: AirplaneId, location: Location) {
        if let Some(mut airplane) = self.store.airplanes().get_mut(id.get()) {
            airplane.point = Some(location.as_api());
        }
    }

    fn update_flight_plan(&self, id: AirplaneId, flight_plan: FlightPlan) {
        if let Some(mut airplane) = self.store.airplanes().get_mut(id.get()) {
            airplane.flight_plan = flight_plan.as_api();
        }
    }

    fn start_game(&self, map: Map) {
        let mut game_started = self.store.game_state().lock();
        *game_started = GameState::Running;

        let mut map_guard = self.store.map().lock();
        *map_guard = map;
    }

    fn reset(&self) {
        self.store.airplanes().clear();

        let mut game_started = self.store.game_state().lock();
        *game_started = GameState::Ready;
    }
}
