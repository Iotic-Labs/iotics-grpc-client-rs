fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/api")
        .build_server(false)
        .format(true)
        .type_attribute(
            ".common.Uri",
            "#[derive(serde::Serialize, serde::Deserialize)]#[serde(rename_all(serialize = \"snake_case\", deserialize = \"snake_case\"))]",
        )
        .type_attribute(
            ".common.StringLiteral",
            "#[derive(serde::Serialize, serde::Deserialize)]#[serde(rename_all(serialize = \"snake_case\", deserialize = \"snake_case\"))]",
        )
        .type_attribute(
            ".common.LangLiteral",
            "#[derive(serde::Serialize, serde::Deserialize)]#[serde(rename_all(serialize = \"snake_case\", deserialize = \"snake_case\"))]",
        )
        .type_attribute(
            ".common.Literal",
            "#[derive(serde::Serialize, serde::Deserialize)]#[serde(rename_all(serialize = \"snake_case\", deserialize = \"snake_case\"))]",
        )
        .type_attribute(
            ".common.Property.Value",
            "#[derive(serde::Serialize, serde::Deserialize)]#[serde(rename_all(serialize = \"snake_case\", deserialize = \"snake_case\"))]",
        )
        .type_attribute(
            ".common.Property",
            "#[derive(serde::Serialize, serde::Deserialize)]#[serde(rename_all(serialize = \"snake_case\", deserialize = \"snake_case\"))]",
        )
        .type_attribute(
            ".common.GeoLocation",
            "#[derive(serde::Serialize, serde::Deserialize)]#[serde(rename_all(serialize = \"snake_case\", deserialize = \"snake_case\"))]",
        )
        .type_attribute(
            ".common.GeoCircle",
            "#[derive(serde::Serialize, serde::Deserialize)]#[serde(rename_all(serialize = \"snake_case\", deserialize = \"snake_case\"))]",
        )
        .type_attribute(
            ".search.SearchRequest.Payload.Filter",
            "#[derive(serde::Serialize, serde::Deserialize)]#[serde(rename_all(serialize = \"snake_case\", deserialize = \"snake_case\"))]",
        )
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
