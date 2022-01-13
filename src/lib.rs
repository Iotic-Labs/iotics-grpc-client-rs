mod helpers;
pub mod properties;

#[allow(clippy::module_inception)]
mod client {
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

pub mod auth_builder;
pub mod common;
pub mod interest;
pub mod search;
pub mod twin;
