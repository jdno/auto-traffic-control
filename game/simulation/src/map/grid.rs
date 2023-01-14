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
