use async_nats;
use kafka_consumer::nats_client::{connect_nats, publish_message, NatsError};

#[tokio::test]
async fn test_connect_nats_invalid() {
    // Use an invalid address to ensure error handling.
    let res = connect_nats("invalid_url").await;
    match res {
        Err(NatsError::ConnectionError(_)) => (),
        _ => panic!("Expected ConnectionError for invalid URL"),
    }
}

// The following test demonstrates mocking the NATS client.
mod fake_client_tests {
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct FakeNatsClient {
        published: Arc<Mutex<Vec<(String, Vec<u8>)>>>,
    }

    impl FakeNatsClient {
        pub fn new() -> Self {
            Self {
                published: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub async fn publish(&self, subject: String, payload: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
            self.published.lock().unwrap().push((subject, payload));
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_publish_message() {
        let fake_client = FakeNatsClient::new();
        let subject = "test.subject".to_string();
        let payload_data = b"test_payload".to_vec();

        let res = fake_client.publish(subject.clone(), payload_data.clone()).await;
        assert!(res.is_ok(), "Fake publish should succeed");

        let published = fake_client.published.lock().unwrap();
        assert_eq!(published.len(), 1);
        assert_eq!(published[0], (subject, payload_data));
    }
}

#[tokio::test]
async fn test_connect_nats_valid() {
    // Use localhost as valid address
    let res = connect_nats("localhost:4222").await;
    assert!(res.is_ok(), "Expected successful connection");
}

#[tokio::test]
async fn test_publish_message_empty_payload() {
    let client = connect_nats("localhost:4222").await.unwrap();
    let res = publish_message(&client, "test.subject".to_string(), &[]).await;
    assert!(res.is_ok(), "Empty payload should be allowed: {:?}", res);
}

#[tokio::test]
async fn test_publish_message_invalid_client() {
    let client = async_nats::connect("localhost:4222").await.unwrap();
    let res = publish_message(&client, "test.subject".to_string(), b"test").await;
    match res {
        Err(NatsError::PublishError(_)) => (),
        _ => panic!("Expected PublishError for invalid client type"),
    }
}

#[tokio::test]
#[ignore = "requires NATS server running"]
async fn test_publish_message_integration() {
    let client = connect_nats("localhost:4222").await.unwrap();
    let subject = "test.subject".to_string();
    let payload = b"integration_test";
    
    let res = publish_message(&client, subject.clone(), payload).await;
    assert!(res.is_ok(), "Publish should succeed: {:?}", res);
}

mod connection_tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_connection_timeout() {
        // This would require adding timeout functionality to connect_nats
        let res = connect_nats("non-existent-server:4222").await;
        match res {
            Err(NatsError::TimeoutError) => (),
            _ => panic!("Expected TimeoutError for non-existent server"),
        }
    }
}