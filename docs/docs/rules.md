# Rules

**Auto Traffic Control** has a few simple rules.

## Failure

The game ends when two airplanes get too close to each other.

A reasonable approximation of _too close_ is less than 32 pixels of distance
between them.

## Scoring

You score a point for each airplane that lands on a matching airport.

_Matching_ means that the airplane and the airport have the same
[`tag`](/docs/api/types#tag).

## Flight Plans

Flight plans are only valid if they match the following constraints. If a flight
plan violates a constraint, the corresponding validation error (in parentheses)
is returned by the API.

- Every node must be within the bounds of the map (`NODE_OUTSIDE_MAP`)
- The next node must be a neighbor of the current node (`INVALID_STEP`)
- The next node must not be the previous node, i.e. airplanes cannot fly back
  and forth between two nodes (`SHARP_TURN`)
- The flight plan must continue with the first node of the previous plan (`INVALID_START`)
- The flight plan must not contain `restricted` nodes (`RESTRICTED_NODE`)
