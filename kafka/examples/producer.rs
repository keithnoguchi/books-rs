//! Simple producer
use futures_util::future::FutureExt;
use tokio::runtime::Runtime;

use rdkafka::{
    config::ClientConfig,
    message::OwnedHeaders,
    producer::{FutureProducer, FutureRecord},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let brokers = args.nth(1).unwrap_or(String::from("localhost:9092"));
    let topic = args.nth(1).unwrap_or(String::from("test"));
    let mut runtime = Runtime::new()?;
    Ok(runtime.block_on(producer(&brokers, &topic)))
}

async fn producer(brokers: &str, topic_name: &str) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = (0..4_096)
        .map(|i| {
            // The send operation on the topic returns a future, that will be completed once the
            // result or failure from Kafka will be received.
            producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().add("header_key", "header_value")),
                    0,
                )
                .map(move |delivery_status| {
                    // This will be executed onw the result is received
                    eprintln!("Delivery status for message {} received", i);
                    delivery_status
                })
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received received.
    for future in futures {
        eprintln!("Future completed. Result: {:?}", future.await);
    }
}
