use std::sync::Arc;

use tonic::{Request, Response, Status};

use atc::v1::{GetMapRequest, GetMapResponse, NodeToPointRequest, NodeToPointResponse, Point};

use crate::api::AsApi;
use crate::map::Node;
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
impl atc::v1::map_service_server::MapService for MapService {
    async fn get_map(
        &self,
        _request: Request<GetMapRequest>,
    ) -> Result<Response<GetMapResponse>, Status> {
        let map = Some(self.store.map().lock().as_api());
        Ok(Response::new(GetMapResponse { map }))
    }

    async fn node_to_point(
        &self,
        request: Request<NodeToPointRequest>,
    ) -> Result<Response<NodeToPointResponse>, Status> {
        if let Some(api_node) = request.into_inner().node {
            let node = Node::new(api_node.longitude, api_node.latitude);
            let point = node.to_location();

            Ok(Response::new(NodeToPointResponse {
                point: Some(Point {
                    x: point.0,
                    y: point.1,
                }),
            }))
        } else {
            Err(Status::invalid_argument("must provide a Node"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tonic::Request;

    use atc::v1::map_service_server::MapService as ServiceTrait;
    use atc::v1::{Airport, GetMapRequest, Node, NodeToPointRequest, Tag};

    use crate::store::Store;

    use super::MapService;

    fn setup() -> (Arc<Store>, MapService) {
        let store = Arc::new(Store::new());
        let service = MapService::new(store.clone());

        (store, service)
    }

    #[tokio::test]
    async fn get_map() {
        let (_, service) = setup();

        let request = Request::new(GetMapRequest {});
        let response = service.get_map(request).await.unwrap();

        let map = response.into_inner().map.unwrap();

        assert_eq!(
            Airport {
                node: Some(Node {
                    longitude: 0,
                    latitude: 0
                }),
                tag: Tag::Red.into()
            },
            map.airport.unwrap()
        );
    }

    #[tokio::test]
    async fn node_to_location_with_center() {
        let (_, service) = setup();

        let request = Request::new(NodeToPointRequest {
            node: Some(Node {
                longitude: 0,
                latitude: 0,
            }),
        });

        let response = service.node_to_point(request).await.unwrap();
        let location = response.into_inner().point.unwrap();

        assert_eq!(0, location.x);
        assert_eq!(0, location.y);
    }

    #[tokio::test]
    async fn node_to_location() {
        let (_, service) = setup();

        let request = Request::new(NodeToPointRequest {
            node: Some(Node {
                longitude: 1,
                latitude: 2,
            }),
        });

        let response = service.node_to_point(request).await.unwrap();
        let location = response.into_inner().point.unwrap();

        assert_eq!(32, location.x);
        assert_eq!(64, location.y);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<MapService>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<MapService>();
    }
}
