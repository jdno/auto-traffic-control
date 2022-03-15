use tonic::{Request, Response, Status};

use atc::v1::{GetGameStateRequest, GetGameStateResponse};

use crate::SharedGameState;

#[derive(Clone, Debug)]
pub struct GameService {
    game_state: SharedGameState,
}

impl GameService {
    pub fn new(game_state: SharedGameState) -> Self {
        Self { game_state }
    }
}

#[tonic::async_trait]
impl atc::v1::game_service_server::GameService for GameService {
    async fn get_game_state(
        &self,
        _request: Request<GetGameStateRequest>,
    ) -> Result<Response<GetGameStateResponse>, Status> {
        let game_state = self.game_state.lock();

        Ok(Response::new(GetGameStateResponse {
            game_state: (*game_state).into(),
        }))
    }
}
