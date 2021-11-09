fn main() -> Result<(), Box<dyn std::error::Error>> {
    let types_to_derive = vec![
        ".common.Uri",
        ".common.StringLiteral",
        ".common.LangLiteral",
        ".common.Literal",
        ".common.Property.Value",
        ".common.Property",
        ".common.GeoLocation",
        ".common.GeoCircle",
        ".common.Value",
        ".common.TwinID",
        ".common.FeedID",
        ".feed.Feed",
        ".feed.UpsertFeedWithMeta",
        ".search.SearchRequest.Payload.Filter",
        ".search.SearchResponse.FeedDetails",
        ".search.SearchResponse.TwinDetails",
    ];
    let derive_ser_der = "#[derive(serde::Serialize, serde::Deserialize)]#[serde(rename_all(serialize = \"camelCase\", deserialize = \"camelCase\"))]";

    let mut builder = tonic_build::configure()
        .out_dir("src/api")
        .build_server(false)
        .format(true);

    for type_to_derive in types_to_derive {
        builder = builder.type_attribute(type_to_derive, derive_ser_der);
    }

    builder.compile(
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
