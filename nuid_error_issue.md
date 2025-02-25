# Issue: nuid Dependency Error

## Description
When building the `kafka_consumer` project, an error is encountered with the `nuid` dependency. The error message indicates that the `nuid` crate is attempting to import `rand::distributions::Alphanumeric`, which cannot be found, and the method `sample_iter` is not available on `OsRng`. This occurs because the version of `nuid` available on crates.io is not compatible with the version of the `rand` crate being used.

Additionally, an attempt was made to override `nuid` using a patch from its Git repository:
```
[patch.crates-io]
nuid = { git = "https://github.com/async-rs/nuid", branch = "master" }
```
However, this failed due to authentication issues when trying to clone the repository.

## Steps to Reproduce
1. Clone the repository.
2. Ensure your `Cargo.toml` includes dependencies for `rdkafka`, `tokio`, and `async-nats`.
3. Run `cargo run` or `cargo build`.
4. Encounter build errors related to the `nuid` crate.

## Error Messages
The error messages include:
- **Unresolved import**:  
  ```
  error[E0432]: unresolved import `rand::distributions`
  ```
- **Missing method `sample_iter`**:  
  ```
  error[E0599]: the method `sample_iter` exists for struct `OsRng` but its trait bounds were not satisfied
  ```

## Observations
- The `nuid` crate might not have been updated to work with the newer versions of `rand`.
- An override using a Git source could resolve this, but the current setup fails due to Git authentication issues.
- One potential workaround is to configure Cargo to use the Git CLI for fetching via `net.git-fetch-with-cli` in your Cargo configuration.

## Proposed Fixes / Workarounds
- **Fix the Dependency**: Use a fork or a patched version of the `nuid` crate that is compatible with the version of `rand` being used.
- **Configure Git Fetching**: Set `net.git-fetch-with-cli = true` in your Cargo configuration to resolve authentication issues with Git.
- **Alternative**: Remove or replace the functionality dependent on `nuid` if it is not critical to your project.

## Additional Information
- **Environment**: macOS (as per project setup)
- **Relevant Documentation**: [Cargo net.git-fetch-with-cli](https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli)

Please update the dependency or configuration as appropriate to resolve these errors.