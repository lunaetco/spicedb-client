# spicedb-client

Auto-generated Rust client for the SpiceDB gRPC API.

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

```rust,ignore
use spicedb_client::authzed::api::v1::{CheckPermissionRequest, PermissionsService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PermissionsService::connect("http://localhost:50051").await?;

    let request = CheckPermissionRequest {
        consistency: None,
        resource: Some(ObjectReference {
            object_type: "document".to_string(),
            object_id: "mydoc".to_string(),
        }),
        permission: "read".to_string(),
        subject: Some(SubjectReference {
            object: Some(ObjectReference {
                object_type: "user".to_string(),
                object_id: "me".to_string(),
            }),
            optional_relation: "".to_string(),
        }),
        context: None,
        with_tracing: false,
    };

    let response = client.check_permission(request).await?;

    println!("Response: {:?}", response);

    Ok(())
}
```

## Documentation

See the [Buf package documentation](https://buf.build/authzed/api/docs/main:authzed.api.v1) for `buf.build/authzed/api`.

## Developing

First, install buf.

```sh
brew install bufbuild/buf/buf
```

Then download the proto files.

```sh
buf export buf.build/authzed/api -o proto
```

And generate the Rust code from the proto.

```sh
cargo run --features=gen gen
```

If any new files are generated, update `lib.rs` to include them.

## Contributing

We welcome contributions!

## License

This project is licensed under [Apache 2.0](LICENSE).

## Acknowledgements

This client is based on the official [AuthZed SpiceDB API](https://docs.authzed.com) and uses [Tonic](https://github.com/hyperium/tonic) for to generate the gRPC client.
