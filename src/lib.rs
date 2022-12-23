// allowed for now because of https://github.com/tokio-rs/prost/issues/332
#![allow(clippy::derive_partial_eq_without_eq)]

mod auth_builder;
mod channel;
mod common;
mod helpers;

include!(concat!(env!("OUT_DIR"), "/client/mod.rs"));

pub mod host;
pub mod input;
pub mod interest;
pub mod properties;
pub mod search;
pub mod twin;

// re-export everything from `auth_builder`, `channel` and `common` in the root of the crate
pub use auth_builder::*;
pub use channel::*;
pub use common::*;
