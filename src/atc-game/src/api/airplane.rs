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
