use std::process::Command;

#[test]
fn test_main_fail_invalid_nats() {
    // Run the binary with an invalid NATS_URL environment variable to force a failure.
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .env("NATS_URL", "invalid_url")
        .env("BROKERS", "localhost:9092")
        .env("GROUP_ID", "test_group")
        .env("KAFKA_USER", "user")
        .env("KAFKA_PASS", "pass")
        .output()
        .expect("Failed to execute process");
    // The main function should return an error due to invalid NATS_URL.
    assert!(!output.status.success(), "Main should exit with failure for invalid NATS_URL");
}