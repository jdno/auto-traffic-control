use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;

use api::{Api, Store};
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

    loop {
        sleep(Duration::from_secs(1));
    }
}
