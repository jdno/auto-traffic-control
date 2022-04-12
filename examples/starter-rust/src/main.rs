use std::ops::Deref;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use pathfinding::prelude::astar;
use tokio_stream::StreamExt;
use tonic::transport::Channel;

use auto_traffic_control::v1::airplane_service_client::AirplaneServiceClient;
use auto_traffic_control::v1::event_service_client::EventServiceClient;
use auto_traffic_control::v1::game_service_client::GameServiceClient;
use auto_traffic_control::v1::get_game_state_response::GameState;
use auto_traffic_control::v1::stream_response::Event;
use auto_traffic_control::v1::{
    AirplaneDetected, GetGameStateRequest, Node, StartGameRequest, StreamRequest,
    UpdateFlightPlanRequest,
};

use crate::map::{Airport, Map};
use crate::point::{Coord, Point};

mod map;
mod point;

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

    let mut map = Rc::new(None);

    while let Some(message) = stream.next().await {
        let event = message.unwrap().event.unwrap();

        match event {
            Event::AirplaneDetected(airplane_detected) => {
                create_flight_plan(&mut airplane_service, airplane_detected, &map).await;
            }
            Event::GameStarted(game_started) => {
                let game_map = game_started.map.unwrap();

                let airports = game_map
                    .airports
                    .iter()
                    .map(|airport| {
                        let node = airport.node.as_ref().unwrap();

                        Airport {
                            coord: Coord(node.longitude, node.latitude, false),
                            tag: airport.tag(),
                        }
                    })
                    .collect();

                let routing_grid = game_map
                    .routing_grid
                    .iter()
                    .map(|node| Coord(node.longitude, node.latitude, node.restricted))
                    .collect();

                map = Rc::new(Some(Map {
                    airports,
                    routing_grid,
                    width: game_map.width,
                    height: game_map.height,
                }));
            }
            Event::GameStopped(game_stopped) => {
                let points = game_stopped.score;
                println!("Finished with {points} points");

                sleep(Duration::from_secs(5));

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
    map: &Rc<Option<Map>>,
) {
    let airplane = airplane_detected.airplane.unwrap();
    let first_hop = airplane
        .flight_plan
        .get(0)
        .expect("expected flight plan to have at least one hop");

    let starting_point = Point {
        x: first_hop.longitude,
        y: first_hop.latitude,
        map: map.clone(),
    };

    let airport = &map
        .deref()
        .as_ref()
        .unwrap()
        .airports
        .iter()
        .find(|airport| airport.tag == airplane.tag())
        .unwrap();

    let destination_point = Point {
        x: airport.coord.0,
        y: airport.coord.1,
        map: map.clone(),
    };

    let route = astar(
        &starting_point,
        |p| p.neighbors(),
        |p| p.distance(&destination_point) / 3,
        |p| p == &destination_point,
    )
    .unwrap()
    .0;

    let route = route
        .iter()
        .map(|point| Node {
            longitude: point.x,
            latitude: point.y,
            restricted: false,
        })
        .collect();

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
