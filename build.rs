fn main() -> Result<(), Box<dyn std::error::Error>> {
    let types_to_derive = vec![
        "Uri",
        "StringLiteral",
        "LangLiteral",
        "Literal",
        "Property",
        "Property.value",
        "GeoLocation",
        "GeoCircle",
        "Value",
        "TwinID",
        "FeedID",
        "Feed",
        "UpsertFeedWithMeta",
        "SearchRequest.Payload.Filter",
        "SearchResponse.FeedDetails",
        "SearchResponse.TwinDetails",
    ];
    let derive_ser_der = "#[derive(serde::Serialize, serde::Deserialize)]#[serde(rename_all(serialize = \"camelCase\", deserialize = \"camelCase\"))]";

    let mut builder = tonic_build::configure()
        .out_dir("src/client")
        .build_server(false)
        .format(true);

    for type_to_derive in types_to_derive {
        builder = builder.type_attribute(type_to_derive, derive_ser_der);
    }

    builder.compile(
        &[
            "proto/google/rpc/status.proto",
            "api/proto/iotics/api/common.proto",
            "api/proto/iotics/api/search.proto",
            "api/proto/iotics/api/twin.proto",
            "api/proto/iotics/api/feed.proto",
            "api/proto/iotics/api/interest.proto",
        ],
        &["api/proto", "proto"],
    )?;

    Ok(())
}
