use lapin::options::{
    BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Connection, ConnectionProperties, Error, ExchangeKind, Result};
use tokio_amqp::LapinTokioExt;

// exchange declare
// queue declare
// bind queue to exchange
// publish
// get

#[tokio::main]
async fn main() -> Result<()> {
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into());
    let conn = Connection::connect(&addr, ConnectionProperties::default().with_tokio()).await?;
    let channel = conn.create_channel().await?;
    let _ = channel.exchange_declare(
        "hello_exchange",
        ExchangeKind::Topic,
        ExchangeDeclareOptions::default(),
        FieldTable::default(),
    );
    let _ = channel.queue_declare(
        "hello_queue",
        QueueDeclareOptions::default(),
        FieldTable::default(),
    );
    let _ = channel.queue_bind(
        "hello_queue",
        "hello_exchange",
        "hello_routing_key",
        QueueBindOptions::default(),
        FieldTable::default(),
    );
    let _ = channel.basic_publish(
        "hello_exchange",
        "hello_routing_key",
        BasicPublishOptions::default(),
        b"Hello world!".to_vec(),
        BasicProperties::default(),
    );
    // channel.basic_get();

    Ok(())
}
