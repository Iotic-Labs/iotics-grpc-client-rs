mod helpers;

#[allow(clippy::module_inception)]
mod api {
    pub mod common;
    pub mod feed;
    pub mod interest;
    pub mod search;
    pub mod twin;

    #[path = ""]
    pub mod google {
        #[path = "google.rpc.rs"]
        pub mod rpc;

        #[path = "google.protobuf.rs"]
        pub mod protobuf;
    }
}

pub mod common;
pub mod interest;
pub mod search;
pub mod twin;
