use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/gen")
        .compile(
            &[
                "authzed/api/v1/permission_service.proto",
                "authzed/api/v1/schema_service.proto",
                "authzed/api/v1/watch_service.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
