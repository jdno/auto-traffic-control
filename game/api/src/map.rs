use std::sync::Arc;

use tonic::{Request, Response, Status};

use auto_traffic_control::v1::{
    GetMapRequest, GetMapResponse, NodeToPointRequest, NodeToPointResponse,
};
use simulation::prelude::*;

use crate::store::Store;

#[derive(Clone, Debug, Default)]
pub struct MapService {
    store: Arc<Store>,
}

impl MapService {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }
}

#[tonic::async_trait]
impl auto_traffic_control::v1::map_service_server::MapService for MapService {
    async fn get_map(
        &self,
        _request: Request<GetMapRequest>,
    ) -> Result<Response<GetMapResponse>, Status> {
        let map = self.store.map().lock().clone();
        Ok(Response::new(GetMapResponse { map }))
    }

    async fn node_to_point(
        &self,
        request: Request<NodeToPointRequest>,
    ) -> Result<Response<NodeToPointResponse>, Status> {
        if let Some(node) = request.into_inner().node {
            let location: Location = (&node).into();

            Ok(Response::new(NodeToPointResponse {
                point: Some(location.into()),
            }))
        } else {
            Err(Status::invalid_argument("must provide a Node"))
        }
    }
}
