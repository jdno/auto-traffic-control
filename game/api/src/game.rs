use std::sync::Arc;

use tonic::{Request, Response, Status};

use auto_traffic_control::v1::{
    GetGameStateRequest, GetGameStateResponse, StartGameRequest, StartGameResponse,
};
use simulation::prelude::*;

use crate::store::{SharedGameState, Store};

#[derive(Clone, Debug)]
pub struct GameService {
    command_bus: Sender<Command>,
    game_state: SharedGameState,
}

impl GameService {
    pub fn new(command_bus: Sender<Command>, store: Arc<Store>) -> Self {
        Self {
            command_bus,
            game_state: store.game_state().clone(),
        }
    }
}

#[tonic::async_trait]
impl auto_traffic_control::v1::game_service_server::GameService for GameService {
    async fn get_game_state(
        &self,
        _request: Request<GetGameStateRequest>,
    ) -> Result<Response<GetGameStateResponse>, Status> {
        let game_state = self.game_state.lock();

        Ok(Response::new(GetGameStateResponse {
            game_state: (*game_state).into(),
        }))
    }

    async fn start_game(
        &self,
        _request: Request<StartGameRequest>,
    ) -> Result<Response<StartGameResponse>, Status> {
        if self.command_bus.send(Command::StartGame).is_err() {
            return Err(Status::internal("failed to queue command"));
        }

        Ok(Response::new(StartGameResponse {}))
    }
}
