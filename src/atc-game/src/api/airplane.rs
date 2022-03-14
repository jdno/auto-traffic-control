use std::sync::Arc;

use tonic::{Request, Response, Status};

use atc::v1::{
    GetAirplaneRequest, GetAirplaneResponse, UpdateFlightPlanRequest, UpdateFlightPlanResponse,
};

use crate::command::CommandSender;
use crate::components::{AirplaneId, FlightPlan, ValidationError};
use crate::map::Tile;
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

        if let Some(_airplane) = self.store.get(&id) {
            let new_flight_plan = request.flight_plan;
            let tiles = new_flight_plan
                .iter()
                .map(|node| Tile::new(node.x, node.y))
                .collect();

            let errors = match FlightPlan::new(tiles) {
                Ok(flight_plan) => {
                    match self
                        .command_bus
                        .send(Command::UpdateFlightPlan(AirplaneId::new(id), flight_plan))
                    {
                        Ok(_) => Vec::new(),
                        Err(_) => return Err(Status::internal("failed to queue command")),
                    }
                }
                Err(errors) => errors
                    .iter()
                    .map(|error| match error {
                        ValidationError::HasSharpTurns => {
                            atc::v1::update_flight_plan_response::Error::HasSharpTurns.into()
                        }
                        ValidationError::NodeOutOfBounds => {
                            atc::v1::update_flight_plan_response::Error::NodeOutOfBounds.into()
                        }
                        ValidationError::NotInLogicalOrder => {
                            atc::v1::update_flight_plan_response::Error::NotInLogicalOrder.into()
                        }
                    })
                    .collect(),
            };

            // TODO: Create different responses for successful and failed updates
            Ok(Response::new(UpdateFlightPlanResponse {
                validation_errors: errors,
            }))
        } else {
            Err(Status::not_found(&format!(
                "No airplane with id {id} was found"
            )))
        }
    }
}
