// use futures::{Stream, StreamExt};
use futures::Stream;
use mqtt::Message;
use paho_mqtt as mqtt;
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

// 异步编程练习
// 本节目标
// 自定义实现Feature(一次异步值), Stream(可重复的异步值), Sink(一次或多次异步值)

struct MQTT {
    client: Option<mqtt::AsyncClient>,
    // client: mqtt::AsyncClient,

    // stream: impl Stream<Item = Option<Message>>,
    // stream: Pin<&'a mut dyn Stream<Item = Option<Message>>>,
    // stream: Option<&'a Pin<&'a mut dyn Stream<Item = Option<Message>>>>,
    // stream: Option<&'a mut dyn Stream<Item = Option<Message>>>,
    stream: Option<Pin<Box<dyn Stream<Item = Option<Message>>>>>,
    // stream: <Pin<Box<dyn Stream<Item = Option<Message>>>>,
    // stream: Option<Box<dyn Stream<Item = Option<Message>>>>,
    // stream: Option<Box<dyn Stream<Item = T>>>,
}

impl MQTT {
    fn new() -> MQTT {
        MQTT {
            client: None,
            // stream: pin
            stream: None::<Pin<Box<dyn Stream<Item = Option<Message>>>>>,
        }
    }

    async fn init(&mut self) -> anyhow::Result<()> {
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri("tcp://localhost:1883")
            .client_id("rust_async_subscribe")
            .finalize();

        self.client = Some(mqtt::AsyncClient::new(create_opts)?);

        let lwt = mqtt::Message::new("test", "Async subscriber lost connection", mqtt::QOS_1);

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(30))
            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            .clean_session(false)
            .will_message(lwt)
            .finalize();

        let cli = self.client.as_mut().unwrap();

        // // Make the connection to the broker
        println!("Connecting to the MQTT server...");
        cli.connect(conn_opts).await?;

        cli.subscribe_many(&["test"], &[1, 1]).await?;
        // let mut stream = cli.get_stream(52);

        // self.stream = Some(Box::new(cli.get_stream(52)));
        self.stream = Some(Box::pin(cli.get_stream(52)));
        // self.stream = Some(Box::new(stream));

        Ok(())
    }

    // fn get_stream(&self) -> &Pin<Box<dyn Stream<Item = Option<Message>>>> {
    //     self.stream.as_ref().unwrap()
    // }
}

impl Stream for MQTT {
    type Item = Option<Message>;
    // type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // self.stream.unwrap()
        let stream = self.stream.as_mut().unwrap();

        match stream.poll_next(cx) {
            Poll::Ready(msg) => {
                if msg.is_none() {
                    return Poll::Ready(None);
                }
                Poll::Ready(msg)
            }
            Poll::Pending => self.poll_next(cx),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut m = MQTT::new();
    m.init().await?;
    // let stream = m.get_stream();
    // while let Some(msg) = stream.next().await {
    //     if let Some(msg) = msg {
    //         println!("received msg: {}", msg);
    //     }
    // }
    // use futures::StreamExt;
    // while let Some(msg) = m.next().await {
    //     if let Some(msg) = msg {
    //         println!("received msg: {}", msg);
    //     }
    // }

    Ok(())
}
