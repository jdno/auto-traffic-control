use crate::map::Tile;

pub fn route_between(start: &Tile, destination: &Tile) -> Vec<Tile> {
    if start == destination {
        return vec![];
    }

    let delta_x = destination.x() - start.x();
    let delta_y = destination.y() - start.y();

    if start.is_neighbor(destination) {
        return vec![*destination];
    }

    let next_x = if delta_x != 0 {
        let direction = delta_x / delta_x.abs();
        start.x() + direction
    } else {
        start.x()
    };

    let next_y = if delta_y != 0 {
        let direction = delta_y / delta_y.abs();
        start.y() + direction
    } else {
        start.y()
    };

    let next_hop = Tile::new(next_x, next_y);

    let mut route = route_between(&next_hop, destination);
    route.append(&mut vec![next_hop]);

    route
}

#[cfg(test)]
mod tests {
    use crate::map::Tile;

    use super::route_between;

    #[test]
    fn route_between_same_point() {
        let start = Tile::new(0, 0);

        let route = route_between(&start, &start);

        assert!(route.is_empty());
    }

    #[test]
    fn route_between_same_axis() {
        let start = Tile::new(0, 0);
        let destination = Tile::new(2, 0);

        let route = route_between(&start, &destination);

        assert_eq!(vec![Tile::new(2, 0), Tile::new(1, 0)], route);
    }

    #[test]
    fn route_between_diagonal_points() {
        let start = Tile::new(0, 0);
        let destination = Tile::new(2, 2);

        let route = route_between(&start, &destination);

        assert_eq!(vec![Tile::new(2, 2), Tile::new(1, 1)], route);
    }

    #[test]
    fn route_between_distant_points() {
        let start = Tile::new(0, 0);
        let destination = Tile::new(3, 1);

        let route = route_between(&start, &destination);

        assert_eq!(
            vec![Tile::new(3, 1), Tile::new(2, 1), Tile::new(1, 1)],
            route
        );
    }
}
