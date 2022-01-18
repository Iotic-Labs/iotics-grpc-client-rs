use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

const CLIENT_MOD: &str = r#"
#[path = ""]
pub mod client {
    #[path = ""]
    pub mod iotics {
        #[path = "iotics.api.rs"]
        pub mod api;
    }
    
    #[path = ""]
    pub mod google {
        #[path = "google.rpc.rs"]
        pub mod rpc;
    
        #[path = "google.protobuf.rs"]
        pub mod protobuf;
    }
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // types that needs to get the serde attributes
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

    // setup the out folder
    let out_dir = env::var("OUT_DIR").expect("Failed get OUT_DIR.");
    let client_path = Path::new(&out_dir).join("client");
    let _ = fs::remove_dir_all(&client_path);
    fs::create_dir(&client_path)?;

    // generate the client mod
    let mod_path = client_path.join("mod.rs");
    let mut mod_file = File::create(mod_path)?;
    mod_file.write_all(CLIENT_MOD.as_bytes())?;

    // configure the builder
    let mut builder = tonic_build::configure()
        .out_dir(&client_path)
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
