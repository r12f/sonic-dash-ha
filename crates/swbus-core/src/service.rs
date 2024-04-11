use std::pin::Pin;
use swbus_contracts::swbus::swbus_service_server::*;
use swbus_contracts::swbus::*;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug, Default)]
pub struct SwbusServiceImpl {}

type SwbusMessageResult<T> = Result<Response<T>, Status>;
type SwbusMessageStream = Pin<Box<dyn Stream<Item = Result<SwbusMessage, Status>> + Send>>;

#[tonic::async_trait]
impl SwbusService for SwbusServiceImpl {
    type StreamMessagesStream = SwbusMessageStream;

    async fn stream_messages(
        &self,
        request: Request<Streaming<SwbusMessage>>,
    ) -> SwbusMessageResult<Self::StreamMessagesStream> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel::<Result<SwbusMessage, Status>>(128);

        let out_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(out_stream) as Self::StreamMessagesStream
        ))
    }
}
