use std::cmp::min;
use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Grid<T> {
    width: u32,
    height: u32,

    elements: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: u32, height: u32, elements: Vec<T>) -> Self {
        Self {
            width,
            height,
            elements,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&T> {
        self.elements.get((y * self.width + x) as usize)
    }

    pub fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn neighbors(&self, x: u32, y: u32) -> Vec<T> {
        let width_range = x.saturating_sub(1)..=min(self.width.saturating_sub(1), x + 1);
        let height_range = y.saturating_sub(1)..=min(self.height.saturating_sub(1), y + 1);

        let mut neighbors = Vec::new();

        for inner_y in height_range {
            // TODO: Refactor to avoid clone
            for inner_x in width_range.clone() {
                if inner_x == x && inner_y == y {
                    continue;
                }

                if let Some(node) = self.get(inner_x, inner_y) {
                    neighbors.push(node.clone());
                }
            }
        }

        neighbors
    }
}

impl<T> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_element() {
        let grid = Grid::new(2, 2, vec![1, 2, 3, 4]);

        assert_eq!(grid.get(0, 0), Some(&1));
        assert_eq!(grid.get(1, 0), Some(&2));
        assert_eq!(grid.get(0, 1), Some(&3));
        assert_eq!(grid.get(1, 1), Some(&4));
    }

    #[test]
    fn get_element_out_of_bounds() {
        let grid = Grid::new(2, 2, vec![1, 2]);

        assert_eq!(grid.get(2, 0), None);
        assert_eq!(grid.get(0, 2), None);
        assert_eq!(grid.get(2, 2), None);
    }

    #[test]
    fn neighbors_without_neighbors() {
        let grid = Grid::new(1, 1, vec![1]);

        let neighbors = grid.neighbors(0, 0);

        assert!(neighbors.is_empty());
    }

    #[test]
    fn neighbors_center() {
        let grid = Grid::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let neighbors = grid.neighbors(1, 1);

        assert_eq!(vec![1, 2, 3, 4, 6, 7, 8, 9], neighbors);
    }

    #[test]
    fn neighbors_top_left() {
        let grid = Grid::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let neighbors = grid.neighbors(0, 0);

        assert_eq!(vec![2, 4, 5], neighbors);
    }

    #[test]
    fn neighbors_edge() {
        let grid = Grid::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let neighbors = grid.neighbors(0, 1);

        assert_eq!(vec![1, 2, 5, 7, 8], neighbors);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Grid<u32>>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Grid<u32>>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Grid<u32>>();
    }
}
