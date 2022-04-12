use tokio_stream::StreamExt;

use auto_traffic_control::v1::event_service_client::EventServiceClient;
use auto_traffic_control::v1::stream_response::Event;
use auto_traffic_control::v1::StreamRequest;

fn should_print(event: &Event) -> bool {
    match event {
        Event::AirplaneCollided(_) => true,
        Event::AirplaneDetected(_) => true,
        Event::AirplaneLanded(_) => true,
        Event::AirplaneMoved(_) => false,
        Event::FlightPlanUpdated(_) => false,
        Event::LandingAborted(_) => true,
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
                let point = airplane.point.unwrap();

                println!(
                    "Airplane detected: {} at {}:{}",
                    airplane.id, point.x, point.y
                );
            }
            Event::AirplaneLanded(airplane_landed) => {
                println!("Airplane landed: {}", airplane_landed.id);
            }
            Event::AirplaneMoved(airplane_moved) => {
                let airplane_id = airplane_moved.id;
                let point = airplane_moved.point.unwrap();

                println!("Airplane moved: {} to {}:{}", airplane_id, point.x, point.y);
            }
            Event::FlightPlanUpdated(flight_plan_updated) => {
                let airplane_id = flight_plan_updated.id;
                let flight_plan = flight_plan_updated.flight_plan;

                println!(
                    "Flight plan updated for {}: {}",
                    airplane_id,
                    flight_plan
                        .iter()
                        .map(|node| format!("{}:{}", node.longitude, node.latitude))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
            Event::LandingAborted(landing_aborted) => {
                println!(
                    "Landing aborted: Airplane {} has the wrong tag",
                    landing_aborted.id
                );
            }
            Event::GameStarted(_) => {
                println!("Game started");
            }
            Event::GameStopped(_) => {
                println!("Game stopped");
            }
        }
    }

    Ok(())
}
