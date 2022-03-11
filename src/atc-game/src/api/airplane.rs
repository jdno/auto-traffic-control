use tonic::{Request, Response, Status};

use atc::v1::{GetAirplaneRequest, GetAirplaneResponse};

use crate::event::EventSender;

#[derive(Clone, Debug)]
pub struct AirplaneService {
    event_sender: EventSender,
}

impl AirplaneService {
    pub fn new(event_sender: EventSender) -> Self {
        Self { event_sender }
    }
}

#[tonic::async_trait]
impl atc::v1::airplane_service_server::AirplaneService for AirplaneService {
    async fn get_airplane(
        &self,
        _request: Request<GetAirplaneRequest>,
    ) -> Result<Response<GetAirplaneResponse>, Status> {
        Err(Status::not_found("No airplane with the given ID was found"))
    }
}
