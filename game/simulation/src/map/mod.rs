use crate::entity::Airport;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub use self::loader::*;
pub use self::location::*;
pub use self::node::*;

mod loader;
mod location;
mod node;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Map {
    name: String,

    width: u32,
    height: u32,

    airports: Vec<Airport>,
    grid: Vec<Arc<Node>>,
}

impl Map {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
