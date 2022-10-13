// allowed for now because of https://github.com/tokio-rs/prost/issues/332
#![allow(clippy::derive_partial_eq_without_eq)]

mod helpers;
pub mod properties;

include!(concat!(env!("OUT_DIR"), "/client/mod.rs"));

pub mod auth_builder;
pub mod common;
pub mod host;
pub mod input;
pub mod interest;
pub mod search;
pub mod twin;
