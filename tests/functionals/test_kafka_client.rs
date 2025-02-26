use kafka_consumer::kafka_client;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_run_consumer_invalid_nats() {
    // Provide an invalid NATS URL to force connect_nats to fail.
    let res = kafka_client::run_consumer("localhost:9092", "test-group", "test-user", "test-pass", "invalid_url").await;
    assert!(res.is_err(), "Expected error from run_consumer with invalid NATS URL");
}

#[tokio::test]
async fn test_run_consumer_timeout() {
    // Use an invalid Kafka broker to cause an error or timeout.
    let fut = kafka_client::run_consumer("invalid_broker", "test-group", "test-user", "test-pass", "invalid_url");
    let res = timeout(Duration::from_secs(1), fut).await;
    // The future should timeout or return an error.
    assert!(res.is_err() || res.unwrap().is_err());
}