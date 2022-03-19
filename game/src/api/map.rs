use tonic::{Request, Response, Status};

use atc::v1::{Location, NodeToLocationRequest, NodeToLocationResponse};

use crate::map::Tile;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MapService;

#[tonic::async_trait]
impl atc::v1::map_service_server::MapService for MapService {
    async fn node_to_location(
        &self,
        request: Request<NodeToLocationRequest>,
    ) -> Result<Response<NodeToLocationResponse>, Status> {
        if let Some(node) = request.into_inner().node {
            let tile = Tile::new(node.x, node.y);
            let point = tile.to_location();

            Ok(Response::new(NodeToLocationResponse {
                location: Some(Location {
                    longitude: point.0,
                    latitude: point.1,
                }),
            }))
        } else {
            Err(Status::invalid_argument("must provide a Node"))
        }
    }
}

#[cfg(test)]
mod tests {
    use tonic::Request;

    use atc::v1::map_service_server::MapService as ServiceTrait;
    use atc::v1::{Node, NodeToLocationRequest};

    use super::MapService;

    #[tokio::test]
    async fn node_to_location_with_center() {
        let request = Request::new(NodeToLocationRequest {
            node: Some(Node { x: 0, y: 0 }),
        });

        let response = MapService.node_to_location(request).await.unwrap();
        let location = response.into_inner().location.unwrap();

        assert_eq!(0, location.longitude);
        assert_eq!(0, location.latitude);
    }

    #[tokio::test]
    async fn node_to_location() {
        let request = Request::new(NodeToLocationRequest {
            node: Some(Node { x: 1, y: 2 }),
        });

        let response = MapService.node_to_location(request).await.unwrap();
        let location = response.into_inner().location.unwrap();

        assert_eq!(32, location.longitude);
        assert_eq!(64, location.latitude);
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
