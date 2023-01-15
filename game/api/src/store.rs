use std::sync::Arc;

use dashmap::DashMap;
use parking_lot::Mutex;

use auto_traffic_control::v1::get_game_state_response::GameState;
use auto_traffic_control::v1::{Airplane, Map};
use simulation::prelude::*;

pub type SharedGameState = Arc<Mutex<GameState>>;
pub type SharedGrid = Arc<Mutex<Option<Grid<Arc<Node>>>>>;
pub type SharedMap = Arc<Mutex<Option<Map>>>;

#[derive(Clone, Debug)]
pub struct Store {
    airplanes: DashMap<String, Airplane>,
    game_state: SharedGameState,
    map: SharedMap,
    grid: SharedGrid,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn daemonize(mut event_bus: Receiver<Event>, store: Arc<Store>) {
        while let Ok(event) = event_bus.recv().await {
            store.update(event);
        }
    }

    pub fn airplanes(&self) -> &DashMap<String, Airplane> {
        &self.airplanes
    }

    pub fn game_state(&self) -> &SharedGameState {
        &self.game_state
    }

    pub fn map(&self) -> &SharedMap {
        &self.map
    }

    pub fn grid(&self) -> &SharedGrid {
        &self.grid
    }

    fn update(&self, event: Event) {
        match event {
            Event::AirplaneDetected(id, location, flight_plan, tag) => {
                self.insert_airplane(id, location, flight_plan, tag)
            }
            Event::AirplaneLanded(id) => self.remove_airplane(id),
            Event::AirplaneMoved(id, location) => self.move_airplane(id, location),
            Event::FlightPlanUpdated(id, flight_plan) => self.update_flight_plan(id, flight_plan),
            Event::GameStarted(airports, grid, width, height) => {
                self.start_game(airports, grid, width, height)
            }
            Event::GameStopped(_) => self.reset(),
            _ => {}
        }
    }

    fn insert_airplane(
        &self,
        id: AirplaneId,
        location: Location,
        flight_plan: FlightPlan,
        tag: Tag,
    ) {
        let tag: auto_traffic_control::v1::Tag = tag.into();

        self.airplanes().insert(
            id.get().into(),
            Airplane {
                id: id.into(),
                point: Some(location.into()),
                flight_plan: flight_plan.into(),
                tag: tag.into(),
            },
        );
    }

    fn remove_airplane(&self, id: AirplaneId) {
        self.airplanes().remove(id.get());
    }

    fn move_airplane(&self, id: AirplaneId, location: Location) {
        if let Some(mut airplane) = self.airplanes().get_mut(id.get()) {
            airplane.point = Some(location.into());
        }
    }

    fn update_flight_plan(&self, id: AirplaneId, flight_plan: FlightPlan) {
        if let Some(mut airplane) = self.airplanes().get_mut(id.get()) {
            airplane.flight_plan = flight_plan.into();
        }
    }

    fn start_game(&self, airports: Vec<Airport>, grid: Grid<Arc<Node>>, width: u32, height: u32) {
        let mut game_started = self.game_state().lock();
        *game_started = GameState::Running;

        let airports = airports.into_iter().map(|airport| airport.into()).collect();
        let routing_grid = grid
            .elements()
            .iter()
            .map(|node| (*node.clone()).into())
            .collect();

        let mut map_guard = self.map().lock();
        *map_guard = Some(Map {
            airports,
            routing_grid,
            width,
            height,
        });

        let mut grid_guard = self.grid().lock();
        *grid_guard = Some(grid);
    }

    fn reset(&self) {
        self.airplanes().clear();

        let mut map_guard = self.map().lock();
        *map_guard = None;

        let mut grid_guard = self.grid().lock();
        *grid_guard = None;

        let mut game_started = self.game_state().lock();
        *game_started = GameState::Ready;
    }
}

impl Default for Store {
    fn default() -> Self {
        Self {
            airplanes: DashMap::new(),
            game_state: Arc::new(Mutex::new(GameState::Ready)),
            map: Arc::new(Mutex::new(None)),
            grid: Arc::new(Mutex::new(None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Store;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Store>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Store>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Store>();
    }
}
