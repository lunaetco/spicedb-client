# spicedb-client

Ergonomic Rust client for the SpiceDB gRPC API.

## Features

- Asynchronous gRPC client for SpiceDB via Tonic
- Generated from the official AuthZed SpiceDB protobuf definitions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
spicedb-client = "0.1.0"
```

## Usage

```rust
use spicedb_client::SpicedbClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spicedb_url = std::env::var("SPICEDB_URL")
        .unwrap_or_else(|_| "http://localhost:50051".to_string());

    let preshared_key = std::env::var("SPICEDB_PRESHARED_KEY")
        .unwrap_or_else(|_| "spicedb".to_string());

    let mut client = SpicedbClient::from_url_and_preshared_key(spicedb_url, preshared_key)
        .await
        .unwrap();

    // Read schema
    let (schema, _token) = client.read_schema().await.unwrap();

    println!("Schema: {:?}", schema);

    Ok(())
}
```

## Documentation

See [docs.rs/spicedb-client](https://docs.rs/spicedb-client).

## Contributing

We welcome contributions!

## License

This project is licensed under [Apache 2.0](LICENSE).

## Acknowledgements

This client is based on the official [AuthZed SpiceDB API](https://docs.authzed.com) and uses [Tonic](https://github.com/hyperium/tonic) for to generate the gRPC client.
