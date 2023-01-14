use simulation::behavior::Updateable;
use simulation::bus::{Command, Event};
use simulation::Simulation;

#[tokio::test]
async fn start_game() {
    let mut simulation = Simulation::new();

    let command_sender = simulation.command_bus();
    let mut event_receiver = simulation.event_bus();

    command_sender.send(Command::StartGame).unwrap();

    simulation.update(0.0);

    let event = event_receiver
        .recv()
        .await
        .expect("failed to receive event");

    assert!(matches!(event, Event::GameStarted(_, _, _, _)));
}
