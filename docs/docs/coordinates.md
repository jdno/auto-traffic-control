# Coordinates

The games uses two different coordinate systems for two different purposes:

- The _position_ of an airplane on the map is represented by a
  [`Point`](/docs/api/types#point).
- The flight plan of an airplane is a list of [`Nodes`](/docs/api/types#node) on
  a routing grid.

Both systems are described in more detail below.

## Coordinate System

The _coordinate system_ is used to describe the position of entities on the map.
It uses the [`Point`](/docs/api/types#point) type, which has an `x` and `y`
field. Every pixel in the game can be addressed with a `Point`.

The origin of the coordinate system is at the center of the map. The x-axis goes
from left to right, and the y-axis goes from the bottom to the top.

![coordinate system](/img/coordinate-system.png)

## Routing Grid

Airplanes are routed along the _routing grid_. The grid consists of
[`Nodes`](/docs/api/types#node) that are distributed in an even pattern across
the map. The following screenshots highlights the nodes that form the routing
grid in red.

![routing grid](/img/routing-grid.png)

The routing grid is part of the [`Map`](/docs/api/types#map), and can be queried
through the [`MapService`](/docs/api/Services/map-service).

Every [`Nodes`](/docs/api/types#node) has a field called `restricted` which
indicates whether an airplane can pass through it. In the screenshot above, the
nodes around the airport are `restricted`, and are thus not rendered on the map.

Airplanes fly from node to node. While in flight, their position is described by
a [`Point`](/docs/api/types#point). But their fligh plan, i.e. their _route_, is
represented by a list of [`Nodes`](/docs/api/types#node).

Since the routing grid overlays the map, the position of every
[`Node`](/docs/api/types#node) can be converted to a
[`Point`](/docs/api/types#point) in the [coordinate system](#coordinate-system).
See the documentation of the [`MapService`](/docs/api/Services/map-service) for
more information.
