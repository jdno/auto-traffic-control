use tokio_stream::StreamExt;

use atc::v1::event_service_client::EventServiceClient;
use atc::v1::stream_response::Event;
use atc::v1::StreamRequest;

fn should_print(event: &Event) -> bool {
    match event {
        Event::AirplaneCollided(_) => true,
        Event::AirplaneDetected(_) => true,
        Event::AirplaneLanded(_) => true,
        Event::AirplaneMoved(_) => false,
        Event::FlightPlanUpdated(_) => false,
        Event::GameStarted(_) => true,
        Event::GameStopped(_) => true,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EventServiceClient::connect("http://localhost:4747").await?;

    let mut stream = client.stream(StreamRequest {}).await.unwrap().into_inner();

    while let Some(message) = stream.next().await {
        let event = message.unwrap().event.unwrap();

        if !should_print(&event) {
            continue;
        }

        match event {
            Event::AirplaneCollided(collision) => {
                println!(
                    "Airplane {} collided with airplane {}",
                    collision.id1, collision.id2
                );
            }
            Event::AirplaneDetected(airplane_detected) => {
                let airplane = airplane_detected.airplane.unwrap();
                let location = airplane.location.unwrap();

                println!(
                    "Airplane detected: {} at {}:{}",
                    airplane.id, location.longitude, location.latitude
                );
            }
            Event::AirplaneLanded(airplane_landed) => {
                println!("Airplane landed: {}", airplane_landed.id);
            }
            Event::AirplaneMoved(airplane_moved) => {
                let airplane_id = airplane_moved.id;
                let location = airplane_moved.location.unwrap();

                println!(
                    "Airplane moved: {} to {}:{}",
                    airplane_id, location.longitude, location.latitude
                );
            }
            Event::FlightPlanUpdated(flight_plan_updated) => {
                let airplane_id = flight_plan_updated.id;
                let flight_plan = flight_plan_updated.flight_plan;

                println!(
                    "Flight plan updated for {}: {}",
                    airplane_id,
                    flight_plan
                        .iter()
                        .map(|node| format!("{}:{}", node.x, node.y))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
            Event::GameStarted(_) => {
                println!("Game started")
            }
            Event::GameStopped(_) => {
                println!("Game stopped")
            }
        }
    }

    Ok(())
}
