use std::sync::Arc;

use clap::Parser;
use game_loop::game_loop;

use api::{Api, Store};
use simulation::behavior::Updateable;
use simulation::Simulation;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Parser)]
struct Args {}

#[tokio::main]
async fn main() {
    let _args = Args::parse();

    let simulation = Simulation::new();
    let store = Arc::new(Store::new());

    tokio::spawn(Store::daemonize(simulation.event_bus(), store.clone()));
    tokio::spawn(Api::serve(
        simulation.command_bus(),
        simulation.event_bus(),
        store,
    ));

    game_loop(simulation, 60, 0.1, |g| g.game.update(1.0 / 60.0), |_g| {});
}
