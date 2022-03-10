use std::pin::Pin;

use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use tonic::codegen::futures_core::Stream;
use tonic::{Request, Response, Status};

use atc::v1::{StreamRequest, StreamResponse};

use crate::api::IntoApi;
use crate::event::EventSender;

#[derive(Clone, Debug)]
pub struct EventService {
    event_sender: EventSender,
}

impl EventService {
    pub fn new(event_sender: EventSender) -> Self {
        Self { event_sender }
    }
}

#[tonic::async_trait]
impl atc::v1::event_service_server::EventService for EventService {
    type StreamStream =
        Pin<Box<dyn Stream<Item = Result<StreamResponse, Status>> + Send + Sync + 'static>>;

    async fn stream(
        &self,
        _request: Request<StreamRequest>,
    ) -> Result<Response<Self::StreamStream>, Status> {
        let stream = BroadcastStream::new(self.event_sender.subscribe()).filter_map(|event| {
            let event = match event {
                Ok(event) => Some(event.into_api()),
                Err(_) => None,
            };

            event.map(|event| Ok(StreamResponse { event: Some(event) }))
        });

        Ok(Response::new(Box::pin(stream) as Self::StreamStream))
    }
}
