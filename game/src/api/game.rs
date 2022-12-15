use std::sync::Arc;

use tonic::{Request, Response, Status};

use auto_traffic_control::v1::{
    GetGameStateRequest, GetGameStateResponse, StartGameRequest, StartGameResponse,
};

use crate::command::{Command, CommandSender};
use crate::store::{SharedGameState, Store};

#[derive(Clone, Debug)]
pub struct GameService {
    command_bus: CommandSender,
    game_state: SharedGameState,
}

impl GameService {
    pub fn new(command_bus: CommandSender, store: Arc<Store>) -> Self {
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
        if self.command_bus.get().send(Command::StartGame).is_err() {
            return Err(Status::internal("failed to queue command"));
        }

        Ok(Response::new(StartGameResponse {}))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::sync::broadcast::channel;
    use tonic::{Code, Request};

    use auto_traffic_control::v1::game_service_server::GameService as ServiceTrait;
    use auto_traffic_control::v1::get_game_state_response::GameState;
    use auto_traffic_control::v1::{GetGameStateRequest, StartGameRequest};

    use crate::command::{Command, CommandReceiver, CommandSender};
    use crate::Store;

    use super::GameService;

    fn setup() -> (CommandReceiver, GameService) {
        let (command_sender, command_receiver) = channel::<Command>(1024);
        let command_sender = CommandSender::new(command_sender);
        let command_receiver = CommandReceiver::new(command_receiver);

        let store = Arc::new(Store::new());

        let service = GameService::new(command_sender, store);

        (command_receiver, service)
    }

    #[tokio::test]
    async fn get_game_state() {
        let (_command_bus, service) = setup();

        let request = Request::new(GetGameStateRequest {});
        let response = service.get_game_state(request).await.unwrap();

        assert_eq!(GameState::Ready, response.into_inner().game_state());
    }

    #[tokio::test]
    async fn start_game_fails_to_queue_command() {
        let (command_bus, service) = setup();
        std::mem::drop(command_bus);

        let request = Request::new(StartGameRequest {});
        let status = service.start_game(request).await.unwrap_err();

        assert_eq!(status.code(), Code::Internal);
    }

    #[tokio::test]
    async fn start_game_queues_command() {
        let (mut command_bus, service) = setup();

        let request = Request::new(StartGameRequest {});
        assert!(service.start_game(request).await.is_ok());

        let command = command_bus.get_mut().try_recv().unwrap();
        assert_eq!(Command::StartGame, command);
    }
}
