use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

use tonic::transport::{Error, Server as GrpcServer};

use atc::v1::airplane_service_server::AirplaneServiceServer;
use atc::v1::event_service_server::EventServiceServer;
use atc::v1::game_service_server::GameServiceServer;

use crate::command::CommandSender;
use crate::event::EventSender;
use crate::store::Store;
use crate::SharedGameState;

use self::airplane::AirplaneService;
use self::event::EventService;
use self::game::GameService;

mod airplane;
mod event;
mod game;

const INTERFACE_VARIABLE: &str = "AUTO_TRAFFIC_CONTROL_INTERFACE";

pub struct Api;

impl Api {
    pub async fn serve(
        command_sender: CommandSender,
        event_sender: EventSender,
        game_state: SharedGameState,
        store: Arc<Store>,
    ) -> Result<(), Error> {
        GrpcServer::builder()
            .add_service(AirplaneServiceServer::new(AirplaneService::new(
                command_sender.clone(),
                store,
            )))
            .add_service(EventServiceServer::new(EventService::new(event_sender)))
            .add_service(GameServiceServer::new(GameService::new(
                command_sender,
                game_state,
            )))
            .serve(Self::address_or_default())
            .await
    }

    fn address_or_default() -> SocketAddr {
        if let Ok(address_string) = std::env::var(INTERFACE_VARIABLE) {
            if let Ok(address) = SocketAddr::from_str(&address_string) {
                return address;
            }
        }

        SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 4747)
    }
}

pub trait IntoApi {
    type ApiType;

    fn into_api(self) -> Self::ApiType;
}
