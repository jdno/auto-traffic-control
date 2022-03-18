use tokio_stream::StreamExt;

use atc::v1::event_service_client::EventServiceClient;
use atc::v1::StreamRequest;

mod route;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EventServiceClient::connect("http://localhost:4747").await?;

    let mut stream = client.stream(StreamRequest {}).await.unwrap().into_inner();

    while let Some(message) = stream.next().await {
        let _event = message.unwrap().event.unwrap();
    }

    Ok(())
}
