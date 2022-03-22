use std::sync::Arc;

use dashmap::DashMap;
use parking_lot::Mutex;

use atc::v1::get_game_state_response::GameState;
use atc::v1::Airplane;

use crate::map::Map;

pub use self::watcher::StoreWatcher;

mod watcher;

pub type SharedGameState = Arc<Mutex<GameState>>;
pub type SharedMap = Arc<Mutex<Map>>;

#[derive(Clone, Debug)]
pub struct Store {
    airplanes: DashMap<String, Airplane>,
    game_state: SharedGameState,
    map: SharedMap,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO: Hide implementation details (DashMap) and provide query interface for airplanes
    pub fn airplanes(&self) -> &DashMap<String, Airplane> {
        &self.airplanes
    }

    pub fn game_state(&self) -> &SharedGameState {
        &self.game_state
    }

    pub fn map(&self) -> &SharedMap {
        &self.map
    }
}

impl Default for Store {
    fn default() -> Self {
        Self {
            airplanes: DashMap::new(),
            game_state: Arc::new(Mutex::new(GameState::Ready)),
            map: Arc::new(Mutex::new(Map::new())),
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
}
