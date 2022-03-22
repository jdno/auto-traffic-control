use tokio_stream::StreamExt;
use tonic::transport::Channel;

use crate::route::route_between;
use atc::v1::airplane_service_client::AirplaneServiceClient;
use atc::v1::event_service_client::EventServiceClient;
use atc::v1::game_service_client::GameServiceClient;
use atc::v1::get_game_state_response::GameState;
use atc::v1::stream_response::Event;
use atc::v1::{
    AirplaneDetected, GetGameStateRequest, Node, StartGameRequest, StreamRequest,
    UpdateFlightPlanRequest,
};

mod route;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut airplane_service = AirplaneServiceClient::connect("http://localhost:4747").await?;
    let mut event_service = EventServiceClient::connect("http://localhost:4747").await?;
    let mut game_service = GameServiceClient::connect("http://localhost:4747").await?;

    let mut stream = event_service
        .stream(StreamRequest {})
        .await
        .unwrap()
        .into_inner();

    let game_state = game_service
        .get_game_state(GetGameStateRequest {})
        .await
        .unwrap()
        .into_inner()
        .game_state();

    if game_state == GameState::Ready {
        start_game(&mut game_service).await;
    }

    while let Some(message) = stream.next().await {
        let event = message.unwrap().event.unwrap();

        match event {
            Event::AirplaneDetected(airplane_detected) => {
                create_flight_plan(&mut airplane_service, airplane_detected).await;
            }
            Event::GameStopped(game_stopped) => {
                let points = game_stopped.score;
                println!("Finished with {points} points");

                start_game(&mut game_service).await;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn start_game(game_service: &mut GameServiceClient<Channel>) {
    game_service.start_game(StartGameRequest {}).await.unwrap();
}

async fn create_flight_plan(
    airplane_service: &mut AirplaneServiceClient<Channel>,
    airplane_detected: AirplaneDetected,
) {
    let airplane = airplane_detected.airplane.unwrap();
    let first_hop = airplane
        .flight_plan
        .get(0)
        .expect("expected flight plan to have at least one hop");

    let destination = Node {
        longitude: 0,
        latitude: 0,
    };
    let route = route_between(first_hop, &destination, true);

    let request = UpdateFlightPlanRequest {
        id: airplane.id,
        flight_plan: route,
    };

    airplane_service
        .update_flight_plan(request)
        .await
        .unwrap()
        .into_inner()
        .payload
        .unwrap();
}
