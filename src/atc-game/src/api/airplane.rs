use std::sync::Arc;

use tonic::{Request, Response, Status};

use atc::v1::{GetAirplaneRequest, GetAirplaneResponse};

use crate::store::Store;

#[derive(Clone, Debug)]
pub struct AirplaneService {
    store: Arc<Store>,
}

impl AirplaneService {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }
}

#[tonic::async_trait]
impl atc::v1::airplane_service_server::AirplaneService for AirplaneService {
    async fn get_airplane(
        &self,
        request: Request<GetAirplaneRequest>,
    ) -> Result<Response<GetAirplaneResponse>, Status> {
        let id = request.into_inner().id;

        if let Some(airplane) = self.store.get(&id) {
            Ok(Response::new(GetAirplaneResponse {
                airplane: Some(airplane.clone()),
            }))
        } else {
            Err(Status::not_found(&format!(
                "No airplane with id {id} was found"
            )))
        }
    }
}
