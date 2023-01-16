use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub use self::airport::*;
pub use self::direction::*;
pub use self::grid::*;
pub use self::loader::*;
pub use self::location::*;
pub use self::node::*;

mod airport;
mod direction;
mod grid;
mod loader;
mod location;
mod node;

pub const MAP_BORDER_WIDTH: u32 = 3;

#[derive(Debug, Default)]
pub struct Map {
    name: String,

    width: u32,
    height: u32,

    airports: Vec<Airport>,
    grid: Grid<Arc<Node>>,
}

impl Map {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn airports(&self) -> &Vec<Airport> {
        &self.airports
    }

    pub fn grid(&self) -> &Grid<Arc<Node>> {
        &self.grid
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use parking_lot::Mutex;

    use super::*;

    impl Map {
        pub fn test() -> Arc<Mutex<Map>> {
            let map = MapLoader::load(Maps::Test);
            Arc::new(Mutex::new(map))
        }
    }

    #[test]
    fn trait_display() {
        let map = Map {
            name: "test".to_string(),
            ..Default::default()
        };

        assert_eq!("test", map.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Map>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Map>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Map>();
    }
}
