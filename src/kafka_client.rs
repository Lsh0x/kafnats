use rdkafka::consumer::{StreamConsumer, Consumer, CommitMode};
use rdkafka::Message;
use rdkafka::config::ClientConfig;
use crate::nats_client::{connect_nats, publish_message};

pub async fn run_consumer(
    brokers: &str, 
    group_id: &str, 
    kafka_user: &str, 
    kafka_pass: &str, 
    nats_url: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to NATS using the nats crate (synchronous API).
    let nats_client = connect_nats(nats_url).await?;
    
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        // Configure SASL authentication for Kafka.
        .set("security.protocol", "SASL_PLAINTEXT")
        .set("sasl.mechanisms", "PLAIN")
        .set("sasl.username", kafka_user)
        .set("sasl.password", kafka_pass)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()?;

    // Subscribe to all topics using a regex pattern.
    consumer.subscribe(&["^.*$"]).expect("Can't subscribe to topics");

    println!("Kafka consumer started. Listening to all topics. Forwarding messages to NATS subject 'kafka.messages'...");

    loop {
        let m = if std::env::var("TESTING").is_ok() {
            match tokio::time::timeout(std::time::Duration::from_secs(1), consumer.recv()).await {
                Ok(Ok(msg)) => Ok(msg),
                Ok(Err(e)) => Err(e),
                Err(_) => break,
            }
        } else {
            consumer.recv().await
        };
        if let Ok(msg) = m {
            if let Some(payload_result) = msg.payload_view::<str>() {
                match payload_result {
                    Ok(payload) => {
                        println!("Received message: {}", payload);
                        let owned_payload = payload.to_owned();
                        let client_clone = nats_client.clone();
                        publish_message(&client_clone, "kafka.messages".to_string(), owned_payload.as_bytes()).await?;
                    },
                    Err(e) => eprintln!("Error decoding payload: {:?}", e),
                }
            } else {
                println!("No payload");
            }
            consumer.commit_message(&msg, CommitMode::Async).unwrap();
        } else {
            break;
        }
    }
    Ok(())
}
