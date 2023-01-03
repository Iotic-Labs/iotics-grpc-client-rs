/*
re-exports to be used by client caller code
not to be used by this library itself
*/
pub use tonic::transport::Channel;
pub use tonic::{Response, Streaming};

pub use crate::client::iotics::api::{
    property::Value, FeedData, FeedId, GeoCircle, GeoLocation, Headers, InputId, LangLiteral,
    Limit, Literal, Offset, Property, PropertyUpdate, Range, Scope, StringLiteral,
    SubscriptionHeaders, TwinId, Uri, Value as FeedValue, Values as FeedValues,
};
