use std::error::Error;

use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::config::ClientConfig;
use rdkafka::message::Message;

use nats;

/// Runs the Kafka subscriber and forwards messages to NATS
/// 
/// # Arguments
/// 
/// * `brokers` - Kafka broker addresses, e.g., "localhost:9092"
/// * `group_id` - Consumer group id, e.g., "example_group"
/// * `nats_server` - NATS server address, e.g., "nats://localhost:4222"
pub fn run(brokers: &str, group_id: &str, nats_server: &str) -> Result<(), Box<dyn Error>> {
    // Create Kafka consumer with the given configuration
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", group_id)
        .set("auto.offset.reset", "earliest")
        .create()?;

    // Fetch metadata to subscribe to all topics
    let metadata = consumer.fetch_metadata(None, std::time::Duration::from_secs(5))?;
    let topics: Vec<&str> = metadata.topics().iter().map(|t| t.name()).collect();
    println!("Discovered topics: {:?}", topics);
    consumer.subscribe(&topics)?;

    // Connect to NATS using the provided server address
    let nc = nats::connect(nats_server)?;

    // Consume messages from Kafka and publish to NATS
    for message in consumer.iter() {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload() {
                    println!("Received message on topic: '{}', partition: {}, offset: {}.", m.topic(), m.partition(), m.offset());
                    nc.publish("subject.example", payload)?;
                    println!("Forwarded message from Kafka to NATS.");
                }
            },
            Err(e) => {
                eprintln!("Error while receiving from Kafka: {:?}", e);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_run() {
        // This test is ignored by default since it requires running Kafka and NATS instances.
        let brokers = "localhost:9092";
        let group_id = "test_group";
        let nats_server = "nats://localhost:4222";
        let result = run(brokers, group_id, nats_server);
        assert!(result.is_ok());
    }
}