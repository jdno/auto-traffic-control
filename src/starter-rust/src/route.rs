use atc::v1::Node;

pub fn route_between(start: &Node, destination: &Node, first_hop: bool) -> Vec<Node> {
    let mut route = if first_hop {
        vec![start.clone()]
    } else {
        Vec::new()
    };

    if start == destination {
        return route;
    }

    let delta_x = destination.x - start.x;
    let delta_y = destination.y - start.y;

    // Start and destination are direct neighbors
    if delta_x.abs() <= 1 && delta_y.abs() <= 1 {
        route.push(destination.clone());
        return route;
    }

    let next_x = if delta_x != 0 {
        let direction = delta_x / delta_x.abs();
        start.x + direction
    } else {
        start.x
    };

    let next_y = if delta_y != 0 {
        let direction = delta_y / delta_y.abs();
        start.y + direction
    } else {
        start.y
    };

    let next_hop = Node {
        x: next_x,
        y: next_y,
    };

    let mut remaining_route = route_between(&next_hop, destination, false);

    route.push(next_hop);
    route.append(&mut remaining_route);

    route
}

#[cfg(test)]
mod tests {
    use atc::v1::Node;

    use super::route_between;

    #[test]
    fn route_between_same_point() {
        let start = Node { x: 0, y: 0 };

        let route = route_between(&start, &start, true);

        assert_eq!(vec![start], route);
    }

    #[test]
    fn route_between_same_axis() {
        let start = Node { x: 0, y: 0 };
        let destination = Node { x: 2, y: 0 };

        let route = route_between(&start, &destination, true);

        assert_eq!(vec![start, Node { x: 1, y: 0 }, destination], route);
    }

    #[test]
    fn route_between_diagonal_points() {
        let start = Node { x: 0, y: 0 };
        let destination = Node { x: 2, y: 2 };

        let route = route_between(&start, &destination, true);

        assert_eq!(vec![start, Node { x: 1, y: 1 }, destination], route);
    }

    #[test]
    fn route_between_distant_points() {
        let start = Node { x: 0, y: 0 };
        let destination = Node { x: 3, y: 1 };

        let route = route_between(&start, &destination, true);

        assert_eq!(
            vec![start, Node { x: 1, y: 1 }, Node { x: 2, y: 1 }, destination],
            route
        );
    }
}
