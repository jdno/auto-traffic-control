use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::entity::{Airplane, Airport};

pub use self::grid::*;
pub use self::loader::*;
pub use self::location::*;
pub use self::node::*;

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
    airplanes: Vec<Airplane>,
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

    #[allow(dead_code)] // TODO: Remove when collision detection is implemented
    pub fn airplanes(&self) -> &Vec<Airplane> {
        &self.airplanes
    }

    pub fn airplanes_mut(&mut self) -> &mut Vec<Airplane> {
        &mut self.airplanes
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
