use std::error::Error;
use std::io::ErrorKind;
use std::sync::Arc;
use std::time::Duration;
use dashmap::DashMap;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{async_trait, Request, Response, Status, Streaming};
use crate::message::message_client::MessageClient;
use crate::message::message_server::{Message, MessageServer};
use crate::message::*;

type InternalConnections<T> = Arc<DashMap<String, mpsc::Sender<Result<T, Status>>>>;

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}


#[derive(Debug, Clone, Default)]
pub struct MessageService {
    internal_connections: InternalConnections<MessageResponse>, // A concurrent map to store channels for each client identified by a unique ID
    id: String,
    message: String,
}

impl MessageService {
    pub fn new() -> Self {
        let msg_service = MessageService {
            internal_connections: Arc::new(DashMap::new()),
            id: "".to_string(),
            message: "".to_string(),
        };

        msg_service
    }
}

#[async_trait]
impl Message for MessageService {
    type SendMessageStream = ReceiverStream<Result<MessageResponse, Status>>;

    async fn send_message(&self, request: Request<Streaming<MessageRequest>>) -> Result<Response<Self::SendMessageStream>, Status> {
        let mut payload = request.into_inner();
        let response: MessageResponse =
            MessageResponse { id: "Internal Server Error".to_owned(), message: "Please try again!".to_owned() };

        let (tx, rx) = mpsc::channel(64); // buffer size of 64 messages

        self.internal_connections.insert("xuankhai".to_owned(), tx.clone());
        println!("{:?}", self.internal_connections);
        tx.send(Ok(response)).await.expect("TODO: panic message");
        if !self.internal_connections.contains_key(&"xuankhai".to_owned()) {
            println!("xuankhai");
            return Ok(Response::new(ReceiverStream::new(rx)));
        }
        while let Some(result) = payload.next().await {
            // println!("{:?}", self.internal_connections.get(&"xuankhai".to_owned()).unwrap());
            match result {
                Ok(v) => self.internal_connections.get(&"xuankhai".to_owned()).unwrap()
                    .send(Ok(MessageResponse { id: v.id, message: v.message }))
                    // .send(Ok(MessageResponse { id: "afasf".to_owned(), message: "asfasf".to_owned() }))
                    .await
                    .expect("working rx"),
                Err(err) => {
                    if let Some(io_err) = match_for_io_error(&err) {
                        if io_err.kind() == ErrorKind::BrokenPipe {
                            // here you can handle special case when client
                            // disconnected in unexpected way
                            eprintln!("\tclient disconnected: broken pipe");
                            break;
                        }
                    }

                    match tx.send(Err(err)).await {
                        Ok(_) => (),
                        Err(_err) => break, // response was dropped
                    }
                }
            }
        }
        println!("{:?}", self.internal_connections);
        Ok(Response::new(ReceiverStream::new(rx)))

        // let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(3000)));
        // tokio::spawn(async move {
        //     while let Some(item) = stream.next().await {
        //         match tx.send(Result::<_, Status>::Ok(item)).await {
        //             Ok(v) => {
        //                 println!("Sending response: {:?}", v);
        //             }
        //             Err(_item) => {
        //                 // output_stream was build from rx and both are dropped
        //                 break;
        //             }
        //         }
        //     }
        //     println!("\tclient disconnected");
        // });

        // tx.send(Result::<_, Status>::Ok(response)).await.unwrap();
        // Ok(Response::new(ReceiverStream::new(rx)))
    }
}