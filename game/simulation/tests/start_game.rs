use simulation::behavior::Updateable;
use simulation::bus::{channel, Command, Event};
use simulation::Simulation;

#[tokio::test]
async fn start_game() {
    let (command_sender, command_receiver) = channel(1);
    let (event_sender, mut event_receiver) = channel(1);

    let mut simulation = Simulation::new(command_receiver, event_sender);

    command_sender.send(Command::StartGame).unwrap();

    simulation.update(0.0);

    let event = event_receiver
        .recv()
        .await
        .expect("failed to receive event");

    assert_eq!(Event::GameStarted, event);
}
