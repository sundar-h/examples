// use anyhow::Result;
use futures::Sink;
use paho_mqtt as mqtt;
use tokio_stream::Stream;

fn main() {}

type Event = String;

/// - Channels
/// - Sockets
/// - Pipes

struct CreateOptions {
    /// The URI for the MQTT broker.
    pub(crate) server_uri: String,
}

#[derive(Default)]
struct ExternalSink {
    cli: Option<mqtt::AsyncClient>,
}

impl ExternalSink {
    fn new() -> Self {
        ExternalSink::default()
    }
}
// impl Sink for ExternalSink {
//     fn init(&mut self, cfg: &str) -> Result<()> {
//         // let cli = mqtt::AsyncClient::new(host).unwrap();
//         // cli.connect(None).wait().unwrap();
//     }

//     fn finalize(&mut self) {
//         todo!()
//     }

//     fn send(&mut self, body: &str) {
//         todo!()
//     }
// }

// impl Sink for ExternalSink {
//     // type Error;

//     fn poll_ready(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         todo!()
//     }

//     fn start_send(self: std::pin::Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
//         todo!()
//     }

//     fn poll_flush(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         todo!()
//     }

//     fn poll_close(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         todo!()
//     }
// }
