pub mod nats_client;
pub mod kafka_client;
use dotenv::dotenv;
use std::env;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let brokers = env::var("BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());
    let group_id = env::var("GROUP_ID").unwrap_or_else(|_| "default-consumer-group".to_string());
    let kafka_user = env::var("KAFKA_USER").unwrap_or_else(|_| "kafka-user".to_string());
    let kafka_pass = env::var("KAFKA_PASS").unwrap_or_else(|_| "kafka-pass".to_string());
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "localhost:4222".to_string());

    kafka_client::run_consumer(&brokers, &group_id, &kafka_user, &kafka_pass, &nats_url).await?;
    Ok(())
}
