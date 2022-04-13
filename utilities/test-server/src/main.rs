use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use semver::Version as SemVer;
use tonic::transport::{Error, Server};
use tonic::{Request, Response, Status};

use auto_traffic_control::v1::atc_service_server::AtcServiceServer;
use auto_traffic_control::v1::{GetVersionRequest, GetVersionResponse, Version};

struct AtcService;

#[tonic::async_trait]
impl auto_traffic_control::v1::atc_service_server::AtcService for AtcService {
    async fn get_version(
        &self,
        _request: Request<GetVersionRequest>,
    ) -> Result<Response<GetVersionResponse>, Status> {
        let semver = SemVer::parse(env!("CARGO_PKG_VERSION")).unwrap();
        let version = Version {
            major: semver.major,
            minor: semver.minor,
            patch: semver.patch,
            pre: semver.pre.to_string(),
        };

        Ok(Response::new(GetVersionResponse {
            version: Some(version),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    Server::builder()
        .add_service(AtcServiceServer::new(AtcService))
        .serve(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(0, 0, 0, 0),
            4747,
        )))
        .await
}
