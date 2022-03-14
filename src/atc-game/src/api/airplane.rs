use std::sync::Arc;

use tonic::{Request, Response, Status};

use atc::v1::update_flight_plan_response::Payload;
use atc::v1::{
    GetAirplaneRequest, GetAirplaneResponse, UpdateFlightPlanError, UpdateFlightPlanRequest,
    UpdateFlightPlanResponse, UpdateFlightPlanSuccess,
};

use crate::command::CommandSender;
use crate::components::{AirplaneId, FlightPlan};
use crate::store::Store;
use crate::Command;

#[derive(Clone, Debug)]
pub struct AirplaneService {
    command_bus: CommandSender,
    store: Arc<Store>,
}

impl AirplaneService {
    pub fn new(command_bus: CommandSender, store: Arc<Store>) -> Self {
        Self { command_bus, store }
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

    async fn update_flight_plan(
        &self,
        request: Request<UpdateFlightPlanRequest>,
    ) -> Result<Response<UpdateFlightPlanResponse>, Status> {
        let request = request.into_inner();
        let id = request.id;

        let airplane = match self.store.get(&id) {
            Some(airplane) => airplane,
            None => {
                return Err(Status::not_found(&format!(
                    "No airplane with id {id} was found"
                )));
            }
        };

        let previous_flight_plan = (&airplane.flight_plan).into();
        let new_flight_plan: FlightPlan = (&request.flight_plan).into();

        if let Err(errors) = new_flight_plan.validate(&previous_flight_plan) {
            let errors = errors.iter().map(|error| (*error).into()).collect();

            return Ok(Response::new(UpdateFlightPlanResponse {
                payload: Some(Payload::Error(UpdateFlightPlanError { errors })),
            }));
        };

        if self
            .command_bus
            .send(Command::UpdateFlightPlan(
                AirplaneId::new(id),
                new_flight_plan,
            ))
            .is_err()
        {
            return Err(Status::internal("failed to queue command"));
        }

        Ok(Response::new(UpdateFlightPlanResponse {
            payload: Some(Payload::Success(UpdateFlightPlanSuccess {})),
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::sync::broadcast::channel;
    use tonic::{Code, Request};

    use atc::v1::airplane_service_server::AirplaneService as ServiceTrait;
    use atc::v1::Airplane;
    use atc::v1::GetAirplaneRequest;

    use crate::api::airplane::AirplaneService;
    use crate::api::IntoApi;
    use crate::command::CommandReceiver;
    use crate::components::{AirplaneId, FlightPlan, Location};
    use crate::{Command, Store};

    fn setup() -> (CommandReceiver, Arc<Store>, AirplaneService) {
        let (command_sender, command_receiver) = channel::<Command>(1024);
        let store = Arc::new(Store::new());
        let service = AirplaneService::new(command_sender, store.clone());

        (command_receiver, store, service)
    }

    #[tokio::test]
    async fn get_airplane_for_existing_plane() {
        let (_command_bus, store, service) = setup();

        let id = AirplaneId::new("AT-4321".into());
        let location = Location::new(0, 0);
        let flight_plan = FlightPlan::new(Vec::new());

        let airplane = Airplane {
            id: id.into_api(),
            location: Some(location.into_api()),
            flight_plan: flight_plan.into_api(),
        };

        store.insert("AT-4321".into(), airplane);

        let request = Request::new(GetAirplaneRequest {
            id: "AT-4321".into(),
        });
        let response = service.get_airplane(request).await.unwrap();

        let payload = response.into_inner();
        let airplane = payload.airplane.unwrap();

        assert_eq!("AT-4321", &airplane.id);
    }

    #[tokio::test]
    async fn get_airplane_with_wrong_id() {
        let (_command_bus, _store, service) = setup();

        let request = Request::new(GetAirplaneRequest {
            id: "AT-4321".into(),
        });
        let status = service.get_airplane(request).await.unwrap_err();

        assert_eq!(status.code(), Code::NotFound);
    }
}
