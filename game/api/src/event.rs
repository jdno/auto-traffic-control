use std::pin::Pin;

use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use tonic::codegen::futures_core::Stream;
use tonic::{Request, Response, Status};

use auto_traffic_control::v1::{StreamRequest, StreamResponse};
use simulation::prelude::*;

#[derive(Debug)]
pub struct EventService {
    event_receiver: Receiver<Event>,
}

impl EventService {
    pub fn new(event_receiver: Receiver<Event>) -> Self {
        Self { event_receiver }
    }
}

#[tonic::async_trait]
impl auto_traffic_control::v1::event_service_server::EventService for EventService {
    type StreamStream =
        Pin<Box<dyn Stream<Item = Result<StreamResponse, Status>> + Send + Sync + 'static>>;

    async fn stream(
        &self,
        _request: Request<StreamRequest>,
    ) -> Result<Response<Self::StreamStream>, Status> {
        let stream =
            BroadcastStream::new(self.event_receiver.resubscribe().into()).filter_map(|event| {
                let event = match event {
                    Ok(event) => Some(event.into()),
                    Err(_) => None,
                };

                event.map(|event| Ok(StreamResponse { event: Some(event) }))
            });

        Ok(Response::new(Box::pin(stream) as Self::StreamStream))
    }
}
