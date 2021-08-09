fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        // .out_dir("src/proto_generated")
        .build_server(false)
        .compile(
            &[
                "proto/search/service.proto",
                "proto/twin/service.proto",
                "proto/feed/service.proto",
                "proto/interest/service.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
