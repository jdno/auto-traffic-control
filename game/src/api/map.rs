use tonic::{Request, Response, Status};

use atc::v1::{NodeToPointRequest, NodeToPointResponse, Point};

use crate::map::Node;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MapService;

#[tonic::async_trait]
impl atc::v1::map_service_server::MapService for MapService {
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
    use tonic::Request;

    use atc::v1::map_service_server::MapService as ServiceTrait;
    use atc::v1::{Node, NodeToPointRequest};

    use super::MapService;

    #[tokio::test]
    async fn node_to_location_with_center() {
        let request = Request::new(NodeToPointRequest {
            node: Some(Node {
                longitude: 0,
                latitude: 0,
            }),
        });

        let response = MapService.node_to_point(request).await.unwrap();
        let location = response.into_inner().point.unwrap();

        assert_eq!(0, location.x);
        assert_eq!(0, location.y);
    }

    #[tokio::test]
    async fn node_to_location() {
        let request = Request::new(NodeToPointRequest {
            node: Some(Node {
                longitude: 1,
                latitude: 2,
            }),
        });

        let response = MapService.node_to_point(request).await.unwrap();
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
