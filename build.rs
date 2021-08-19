fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/api")
        .build_server(false)
        .format(true)
        .compile(
            &[
                "proto/common/service.proto",
                "proto/search/service.proto",
                "proto/twin/service.proto",
                "proto/feed/service.proto",
                "proto/interest/service.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
