// allowed for now because of https://github.com/tokio-rs/prost/issues/332
#![allow(clippy::derive_partial_eq_without_eq)]

mod channel;
mod common;
mod helpers;
pub mod properties;

include!(concat!(env!("OUT_DIR"), "/client/mod.rs"));

pub mod auth_builder;
pub mod host;
pub mod input;
pub mod interest;
pub mod search;
pub mod twin;

// re-export everything from common and channel in the crate root
pub use channel::*;
pub use common::*;
