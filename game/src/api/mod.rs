use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

use tonic::transport::{Error, Server as GrpcServer};

use ::atc::v1::airplane_service_server::AirplaneServiceServer;
use ::atc::v1::atc_service_server::AtcServiceServer;
use ::atc::v1::event_service_server::EventServiceServer;
use ::atc::v1::game_service_server::GameServiceServer;
use ::atc::v1::map_service_server::MapServiceServer;

use crate::command::CommandSender;
use crate::event::EventSender;
use crate::store::Store;

use self::airplane::AirplaneService;
use self::atc::AtcService;
use self::event::EventService;
use self::game::GameService;
use self::map::MapService;

mod airplane;
mod atc;
mod event;
mod game;
mod map;

const INTERFACE_VARIABLE: &str = "AUTO_TRAFFIC_CONTROL_INTERFACE";

pub struct Api;

impl Api {
    pub async fn serve(
        command_sender: CommandSender,
        event_sender: EventSender,
        store: Arc<Store>,
    ) -> Result<(), Error> {
        GrpcServer::builder()
            .add_service(AirplaneServiceServer::new(AirplaneService::new(
                command_sender.clone(),
                store.clone(),
            )))
            .add_service(AtcServiceServer::new(AtcService))
            .add_service(EventServiceServer::new(EventService::new(event_sender)))
            .add_service(GameServiceServer::new(GameService::new(
                command_sender,
                store.clone(),
            )))
            .add_service(MapServiceServer::new(MapService::new(store)))
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

pub trait AsApi {
    type ApiType;

    fn as_api(&self) -> Self::ApiType;
}
