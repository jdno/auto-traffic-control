use std::ops::Deref;
use std::sync::Arc;

use tonic::{Request, Response, Status};

use auto_traffic_control::v1::update_flight_plan_error::ValidationError;
use auto_traffic_control::v1::update_flight_plan_response::Payload;
use auto_traffic_control::v1::{
    GetAirplaneRequest, GetAirplaneResponse, UpdateFlightPlanError, UpdateFlightPlanRequest,
    UpdateFlightPlanResponse, UpdateFlightPlanSuccess,
};
use simulation::prelude::*;

use crate::store::Store;

#[derive(Clone, Debug)]
pub struct AirplaneService {
    command_bus: Sender<Command>,
    store: Arc<Store>,
}

impl AirplaneService {
    pub fn new(command_bus: Sender<Command>, store: Arc<Store>) -> Self {
        Self { command_bus, store }
    }
}

#[tonic::async_trait]
impl auto_traffic_control::v1::airplane_service_server::AirplaneService for AirplaneService {
    async fn get_airplane(
        &self,
        request: Request<GetAirplaneRequest>,
    ) -> Result<Response<GetAirplaneResponse>, Status> {
        let id = request.into_inner().id;

        if let Some(airplane) = self.store.airplanes().get(&id) {
            Ok(Response::new(GetAirplaneResponse {
                airplane: Some(airplane.clone()),
            }))
        } else {
            Err(Status::not_found(format!(
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

        let airplane = match self.store.airplanes().get(&id) {
            Some(airplane) => airplane,
            None => {
                return Err(Status::not_found(format!(
                    "No airplane with id {id} was found"
                )));
            }
        };

        let previous_flight_plan = airplane.flight_plan.clone().into();
        let new_flight_plan: FlightPlan = request.flight_plan.into();

        let grid_guard = self.store.grid().lock();
        let grid = match grid_guard.deref() {
            Some(grid) => grid,
            None => {
                return Err(Status::not_found(
                    "No grid was found. The game has not started yet.".to_string(),
                ));
            }
        };

        if let Err(errors) = new_flight_plan.validate(&previous_flight_plan, grid) {
            let errors = errors
                .iter()
                .map(|error| {
                    let error: ValidationError = (*error).into();
                    error.into()
                })
                .collect();

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
