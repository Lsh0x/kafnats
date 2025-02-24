use env_logger;
use kafnats::subscriber;

fn main() {
    env_logger::init();
    println!("Starting Kafka to NATS subscriber...");

    // Define configuration parameters
    let brokers = "localhost:9092";
    let group_id = "example_group";
    let nats_server = "nats://localhost:4222";

    if let Err(e) = subscriber::run(brokers, group_id, nats_server) {
        eprintln!("Error: {:?}", e);
    }
}