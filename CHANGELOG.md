<!-- markdownlint-disable-file MD013 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1](https://github.com/jdno/atc/releases/tag/v0.3.1)

### Fixed

- Re-export missing ServiceError type by [@jdno](https://github.com/jdno) in [#127](https://github.com/jdno/auto-traffic-control/pull/127)

**Full Changelog**: <https://github.com/jdno/auto-traffic-control/compare/v0.3.0...v0.3.1>

## [0.3.0](https://github.com/jdno/atc/releases/tag/v0.3.0)

### Added

- Publish Node SDK by [@jdno](https://github.com/jdno) in [#97](https://github.com/jdno/auto-traffic-control/pull/97)
- Write tutorial for a TypeScript bot by [@jdno](https://github.com/jdno) in [#104](https://github.com/jdno/auto-traffic-control/pull/104)

### Changed

- Rename enum variants in Protocol Buffer by [@jdno](https://github.com/jdno) in [#99](https://github.com/jdno/auto-traffic-control/pull/99)

**Full Changelog**: <https://github.com/jdno/auto-traffic-control/compare/v0.2.0...v0.3.0>

## [0.2.0](https://github.com/jdno/atc/releases/tag/v0.2.0)

### Added

- Create and add routing grid to the map by [@jdno](https://github.com/jdno) in [#37](https://github.com/jdno/auto-traffic-control/pull/37)
- Create API to query the map by [@jdno](https://github.com/jdno) in [#39](https://github.com/jdno/auto-traffic-control/pull/39)
- Add a game over screen by [@jdno](https://github.com/jdno) in [#41](https://github.com/jdno/auto-traffic-control/pull/41)
- Add routing grid to map data in API responses by [@jdno](https://github.com/jdno) in [#42](https://github.com/jdno/auto-traffic-control/pull/42)
- Add tags to airplanes and airports by [@jdno](https://github.com/jdno) in [#44](https://github.com/jdno/auto-traffic-control/pull/44)
- Add flag to nodes to indicate whether they are accessible by [@jdno](https://github.com/jdno) in [#45](https://github.com/jdno/auto-traffic-control/pull/45)
- Add second airport to the map by [@jdno](https://github.com/jdno) in [#46](https://github.com/jdno/auto-traffic-control/pull/46)
- Add width and height to map message type by [@jdno](https://github.com/jdno) in [#47](https://github.com/jdno/auto-traffic-control/pull/47)
- Prototype path finding in Rust bot by [@jdno](https://github.com/jdno) in [#48](https://github.com/jdno/auto-traffic-control/pull/48)
- Prototype graphics by [@jdno](https://github.com/jdno) in [#50](https://github.com/jdno/auto-traffic-control/pull/50)
- Write getting started guide by [@jdno](https://github.com/jdno) in [#71](https://github.com/jdno/auto-traffic-control/pull/71)
- Document API by [@jdno](https://github.com/jdno) in [#72](https://github.com/jdno/auto-traffic-control/pull/72)
- Document the game rules by [@jdno](https://github.com/jdno) in [#73](https://github.com/jdno/auto-traffic-control/pull/73)
- Publish Rust client by [@jdno](https://github.com/jdno) in [#74](https://github.com/jdno/auto-traffic-control/pull/74)
- Document project status and API versioning by [@jdno](https://github.com/jdno) in [#78](https://github.com/jdno/auto-traffic-control/pull/78)
- Document coordinate systems by [@jdno](https://github.com/jdno) in [#83](https://github.com/jdno/auto-traffic-control/pull/83)

### Changed

- Refactor store to manage all shared state by [@jdno](https://github.com/jdno) in [#38](https://github.com/jdno/auto-traffic-control/pull/38)
- Refactor airplane lifecycle by [@jdno](https://github.com/jdno) in [#40](https://github.com/jdno/auto-traffic-control/pull/40)
- Create type for airports and add it to the map by [@jdno](https://github.com/jdno) in [#43](https://github.com/jdno/auto-traffic-control/pull/43)
- Rename validation errors by [@jdno](https://github.com/jdno) in [#55](https://github.com/jdno/auto-traffic-control/pull/55)
- Relicense project under MIT or Apache-2.0 by [@jdno](https://github.com/jdno) in [#59](https://github.com/jdno/auto-traffic-control/pull/59)
- Add high-res spritesheets by [@jdno](https://github.com/jdno) in [#69](https://github.com/jdno/auto-traffic-control/pull/69)
- Restyle the main screen by [@jdno](https://github.com/jdno) in [#70](https://github.com/jdno/auto-traffic-control/pull/70)

### Fixed

- Normalize speed when flying diagonally by [@jdno](https://github.com/jdno) in [#51](https://github.com/jdno/auto-traffic-control/pull/51)

**Full Changelog**: <https://github.com/jdno/auto-traffic-control/compare/v0.1.0...v0.2.0>

## [0.1.0](https://github.com/jdno/atc/releases/tag/v0.1.0)

### Added

- Prototype systems that render the routing grid by [@jdno](https://github.com/jdno) in [#4](https://github.com/jdno/atc/pull/4)
- Prototype system that follows flight plans by [@jdno](https://github.com/jdno) in [#5](https://github.com/jdno/atc/pull/5)
- Create system that despawns airplanes by [@jdno](https://github.com/jdno) in [#6](https://github.com/jdno/atc/pull/6)
- Prototype path finding for airplanes by [@jdno](https://github.com/jdno) in [#7](https://github.com/jdno/atc/pull/7)
- Prototype API specification by [@jdno](https://github.com/jdno) in [#9](https://github.com/jdno/atc/pull/9)
- Create event service by [@jdno](https://github.com/jdno) in [#10](https://github.com/jdno/atc/pull/10)
- Crate airplane service by [@jdno](https://github.com/jdno) in [#11](https://github.com/jdno/atc/pull/11)
- Create command bus by [@jdno](https://github.com/jdno) in [#13](https://github.com/jdno/atc/pull/13)
- Validate flight plans by [@jdno](https://github.com/jdno) in [#14](https://github.com/jdno/atc/pull/14)
- Create API to update flight plans by [@jdno](https://github.com/jdno) in [#12](https://github.com/jdno/atc/pull/12)
- Create app states and an API to start a game by [@jdno](https://github.com/jdno) in [#16](https://github.com/jdno/atc/pull/16)
- Add collisions by [@jdno](https://github.com/jdno) in [#20](https://github.com/jdno/atc/pull/20)
- Spawn airplanes with random flight plan by [@jdno](https://github.com/jdno) in [#22](https://github.com/jdno/atc/pull/22)
- Create API to inspect ATC itself by [@jdno](https://github.com/jdno) in [#28](https://github.com/jdno/atc/pull/28)

### Changed

- Refactor location and node in API specification by [@jdno](https://github.com/jdno) in [#23](https://github.com/jdno/atc/pull/23)

### Fixed

- Fix validation of flight plans by [@jdno](https://github.com/jdno) in [#25](https://github.com/jdno/atc/pull/25)

**Full Changelog**: <https://github.com/jdno/atc/compare/v0.0.0...v0.1.0>
