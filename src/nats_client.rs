use async_nats;

pub async fn connect_nats(nats_url: &str) -> Result<async_nats::Client, Box<dyn std::error::Error>> {
    let client = async_nats::connect(nats_url).await?;
    Ok(client)
}

pub async fn publish_message(client: &async_nats::Client, subject: String, payload: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    client.publish(subject, payload.to_vec().into()).await?;
    Ok(())
}