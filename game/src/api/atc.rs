use semver::Version as SemVer;
use tonic::{Request, Response, Status};

use atc::v1::{GetVersionRequest, GetVersionResponse, Version};

pub struct AtcService;

#[tonic::async_trait]
impl atc::v1::atc_service_server::AtcService for AtcService {
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

#[cfg(test)]
mod tests {
    use tonic::Request;

    use atc::v1::atc_service_server::AtcService as ServiceTrait;
    use atc::v1::GetVersionRequest;

    use super::AtcService;

    #[tokio::test]
    async fn get_version() {
        let response = AtcService
            .get_version(Request::new(GetVersionRequest {}))
            .await
            .unwrap();

        let version = response.into_inner().version.unwrap();

        assert_eq!(
            env!("CARGO_PKG_VERSION_MAJOR").parse::<u64>().unwrap(),
            version.major
        );
        assert_eq!(
            env!("CARGO_PKG_VERSION_MINOR").parse::<u64>().unwrap(),
            version.minor
        );
        assert_eq!(
            env!("CARGO_PKG_VERSION_PATCH").parse::<u64>().unwrap(),
            version.patch
        );
        assert_eq!(env!("CARGO_PKG_VERSION_PRE"), version.pre);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<AtcService>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<AtcService>();
    }
}
