use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NatsError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),
    #[error("Publish failed: {0}")]
    PublishError(String),
    #[error("Timeout while connecting to NATS server")]
    TimeoutError,
}


use async_nats;

pub async fn connect_nats(nats_url: &str) -> Result<async_nats::Client, NatsError> {
    let connect_result = tokio::time::timeout(
        Duration::from_secs(5),
        async_nats::connect(nats_url)
    ).await;

    match connect_result {
        Ok(Ok(client)) => Ok(client),
        Ok(Err(e)) => Err(NatsError::ConnectionError(e.to_string())),
        Err(_) => Err(NatsError::TimeoutError),
    }
}

pub async fn publish_message(client: &async_nats::Client, subject: String, payload: &[u8]) -> Result<(), NatsError> {
    client.publish(subject, payload.to_vec().into())
        .await
        .map_err(|e| NatsError::PublishError(e.to_string()))
}