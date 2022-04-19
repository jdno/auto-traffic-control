use auto_traffic_control::v1::atc_service_client::AtcServiceClient;
use auto_traffic_control::v1::GetVersionRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut atc_service = AtcServiceClient::connect("http://localhost:4747").await?;

    let response = atc_service.get_version(GetVersionRequest {}).await?;
    let version_field = response.into_inner().version;

    if let Some(version) = version_field {
        let mut version_string = format!("{}.{}.{}", version.major, version.minor, version.patch);

        if !version.pre.is_empty() {
            version_string.push('-');
            version_string.push_str(&version.pre);
        }

        println!("Auto Traffic Control is running version '{version_string}'");
    } else {
        panic!("Requesting the version returned an empty response.");
    }

    Ok(())
}
