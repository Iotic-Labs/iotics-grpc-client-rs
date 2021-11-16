// ---------------------------------------------------------------------------------------------------------------------

/// Limit is a request parameter to limit the number of results.
/// The use of "Limit" is:
/// - if limit is not supplied or supplied with no limit value, assume default=500
/// - if limit is supplied assume max number of entries is whatever specified. The limit value is capped to 1000.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Limit {
    /// Max number of results
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
/// Offset is a request parameter applicable in conjunction with the "Limit"
/// request parameter to start returning results from the given offset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Offset {
    /// Result number to start from
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
/// Range is the combination of the "Limit" and "Offset" is a request parameters. It is
/// used to return a specific range of results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    #[prost(message, optional, tag = "1")]
    pub limit: ::core::option::Option<Limit>,
    #[prost(message, optional, tag = "2")]
    pub offset: ::core::option::Option<Offset>,
}
/// LangLiteral is a metadata property type describing a string with a given language (implicit datatype: rdf:langString).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LangLiteral {
    /// 2-character language code
    #[prost(string, tag = "1")]
    pub lang: ::prost::alloc::string::String,
    /// String representation of the value
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// StringLiteral is a metadata property type describing a string without a language (implicit datatype: rdf:string).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringLiteral {
    /// String representation of the value
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Literal is a metadata property type describing a literal with the given datatype (implicit datatype: rdfs:Literal).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Literal {
    /// XSD data type (e.g. double) without its namespace prefix (<http://www.w3.org/2001/XMLSchema#>). The following types
    /// are currently supported:
    /// dateTime, time, date, boolean, integer, decimal, float, double, nonPositiveInteger, negativeInteger,
    /// nonNegativeInteger, positiveInteger, long, unsignedLong, int, unsignedInt, short, unsignedShort, byte,
    /// unsignedByte, base64Binary, anyURI
    #[prost(string, tag = "1")]
    pub data_type: ::prost::alloc::string::String,
    /// String representation of the value according to XSD datatype specification
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Uri is a metadata property type describing am Uri.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uri {
    /// String representation of the value
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Property is a metadata property with a single value.
/// Multiple instances are used to represent a key (predicate) with multiple values.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// The key (predicate) of the property
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The value of the property. Only one of these fields can be set
    #[prost(oneof = "property::Value", tags = "2, 3, 4, 5")]
    pub value: ::core::option::Option<property::Value>,
}
/// Nested message and enum types in `Property`.
pub mod property {
    /// The value of the property. Only one of these fields can be set
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "2")]
        LiteralValue(super::Literal),
        #[prost(message, tag = "3")]
        LangLiteralValue(super::LangLiteral),
        #[prost(message, tag = "4")]
        StringLiteralValue(super::StringLiteral),
        #[prost(message, tag = "5")]
        UriValue(super::Uri),
    }
}
/// GeoLocation is the geographic location of a Twin.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoLocation {
    /// Latitude
    #[prost(double, tag = "1")]
    pub lat: f64,
    /// Longitude
    #[prost(double, tag = "2")]
    pub lon: f64,
}
/// GeoCircle is an approximate geographic location.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoCircle {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<GeoLocation>,
    /// Radius (Km) relative to the geolocation
    #[prost(double, tag = "2")]
    pub radius_km: f64,
}
/// LabelUpdate describes labels metadata property update: addition and deletion of labels with language.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelUpdate {
    /// List of labels to be added. (Note: Only one label per language is stored, i.e. any existing ones with the same
    /// language will be replaced.)
    #[prost(message, repeated, tag = "1")]
    pub added: ::prost::alloc::vec::Vec<LangLiteral>,
    /// List of languages for which to remove labels
    #[prost(string, repeated, tag = "2")]
    pub deleted_by_lang: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CommentUpdate describes comments metadata property update: addition and deletion of comments with language.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentUpdate {
    /// List of labels to be added. (Note: Only one comment per language is stored, i.e. any existing ones with the same
    /// language will be replaced.)
    #[prost(message, repeated, tag = "1")]
    pub added: ::prost::alloc::vec::Vec<LangLiteral>,
    /// List of languages for which to remove comments
    #[prost(string, repeated, tag = "2")]
    pub deleted_by_lang: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Tags describes tags metadata property update: list of tags to be added and deleted. if a tag appears in both lists,
/// applications may choose to ignore the tags or throw some error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tags {
    /// List of tags to be added
    #[prost(string, repeated, tag = "1")]
    pub added: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of tags to be deleted
    #[prost(string, repeated, tag = "2")]
    pub deleted: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Headers describes the common headers applicable to all the API requests
/// (except for Search subscribe: see SubscriptionHeaders).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Headers {
    /// Optional client reference. Any responses associated with the request will include this reference.
    #[prost(string, tag = "1")]
    pub client_ref: ::prost::alloc::string::String,
    /// User namespace used to group all the requests/responses
    #[prost(string, tag = "2")]
    pub client_app_id: ::prost::alloc::string::String,
    /// Used to loosely link requests/responses in a distributed environment
    /// each layer can add its own id to the list. Transaction ref is limited to
    /// a max of 16 elements per list and, for each list item, a max length of 36
    /// characters
    #[prost(string, repeated, tag = "3")]
    pub transaction_ref: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Used for group listener, optional - Not implemented yet
    #[prost(message, optional, tag = "4")]
    pub consumer_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Client request timeout used to stop the request processing once the timeout is reached
    #[prost(message, optional, tag = "5")]
    pub request_timeout: ::core::option::Option<::prost_types::Timestamp>,
}
/// SubscriptionHeaders describes a Search subscribe header. (Will be DEPRECATED with the client-ref from Headers).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionHeaders {
    /// User namespace used to group all the requests/responses
    #[prost(string, tag = "1")]
    pub client_app_id: ::prost::alloc::string::String,
    /// Used to loosely link requests/responses in a distributed environment
    /// each layer can add its own id to the list. Transaction ref is limited to
    /// a max of 16 elements per list and, for each list item, a max length of 36
    /// characters
    #[prost(string, repeated, tag = "2")]
    pub transaction_ref: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// consumer group (for group listener, optional) - Not implemented yet
    #[prost(message, optional, tag = "3")]
    pub consumer_group: ::core::option::Option<::prost::alloc::string::String>,
}
/// HostID is a unique host identifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostId {
    /// Host Identifier string representation
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// TwinID is a unique twin identifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwinId {
    /// Twin Identifier (using DID format)
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// FeedID is a unique feed identifier (scoped to the TwinID).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedId {
    /// Feed Identifier string representation (simple string)
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Value is the definition of an individual piece of data within a Feed share. Values are purely descriptive, e.g. a
/// Feed follower should expect data to match the values associated with said Feed but must be able to recover where this
/// is not the case.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// label is the unique identifier of the value. It is language-neutral. E.g.: "t" / "temp" / "temperature".
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    /// comment is the (optional) human-readable description of the value. It is language-specific. E.g.: "Engine oil temperature"
    #[prost(string, tag = "2")]
    pub comment: ::prost::alloc::string::String,
    /// unit is the (optional) fully qualified ontology string URI of the unit, e.g. <http://purl.obolibrary.org/obo/UO_0000027>
    #[prost(string, tag = "3")]
    pub unit: ::prost::alloc::string::String,
    /// dataType is the xsd type in shorthand notation.
    /// Currently supported types are: base64Binary, decimal, float, double, dateTime, time, date, boolean, integer,
    /// nonPositiveInteger, negativeInteger, nonNegativeInteger, positiveInteger, long, unsignedLong, int, unsignedInt,
    /// short, unsignedShort, byte, unsignedByte
    #[prost(string, tag = "4")]
    pub data_type: ::prost::alloc::string::String,
}
/// Values defines a set of values to be added and/or deleted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Values {
    /// added is the list of values to be added. Note that deletedByLabel takes precedence over this, i.e. if both added
    /// and deletedByLabel contain the same value, the value will be deleted.
    #[prost(message, repeated, tag = "1")]
    pub added: ::prost::alloc::vec::Vec<Value>,
    /// deletedByLabel is the list of labels of values to be deleted.
    #[prost(string, repeated, tag = "2")]
    pub deleted_by_label: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// FeedData is set of datapoints shared via a Feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedData {
    /// occurredAt is the UTC timestamp of the sample. Typically this is either the time at which an application shared
    /// this sample or the time applicable to the sample itself (such as an hourly weather observation).
    #[prost(message, optional, tag = "2")]
    pub occurred_at: ::core::option::Option<::prost_types::Timestamp>,
    /// mime is the mime type of the encoded data.
    #[prost(string, tag = "3")]
    pub mime: ::prost::alloc::string::String,
    /// data is the actual set of datapoints, encoded according the the mime type. The data should follow the Feed's
    /// value defintions but that is not enforced. (See also Value)
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// PointType used to describe a point as a FEED or a CONTROL.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PointType {
    Feed = 0,
    Control = 1,
}
/// Visibility defines who a twin is visible to.
/// PRIVATE - the twin is only visible in a LOCAL scope.
/// PUBLIC - the twin is visible in any scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Visibility {
    Private = 0,
    Public = 1,
}
/// Scope is a request parameter used to apply a scope to a given request.
/// GLOBAL - go over the network/target the public Twin
/// LOCAL - restrain the request to the local host
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Scope {
    Global = 0,
    Local = 1,
}
/// A feed representation.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    /// feed identifier (unique within the scope of a twin identifier)
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<FeedId>,
    /// twin unique identifier (twin to which the feed belongs)
    #[prost(message, optional, tag = "2")]
    pub twin_id: ::core::option::Option<TwinId>,
}
/// CreateFeedRequestCreate is used to create a new feed in a given twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeedRequest {
    /// CreateFeedRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// CreateFeedRequest mandatory arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<create_feed_request::Arguments>,
    /// CreateFeedRequest payload
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<create_feed_request::Payload>,
}
/// Nested message and enum types in `CreateFeedRequest`.
pub mod create_feed_request {
    /// Payload describes the data needed to create a feed.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// ID of the feed to create
        #[prost(message, optional, tag = "1")]
        pub feed_id: ::core::option::Option<super::FeedId>,
        /// StoreLast indicates if the last received value should be stored of not
        #[prost(bool, tag = "2")]
        pub store_last: bool,
    }
    /// Arguments describes the mandatory arguments to identify the twin the feed belongs to.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// Identifier of the twin owning this feed
        #[prost(message, optional, tag = "1")]
        pub twin_id: ::core::option::Option<super::TwinId>,
    }
}
/// CreateFeedResponse describes a created feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeedResponse {
    /// CreateFeedResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// CreateFeedResponse payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<create_feed_response::Payload>,
}
/// Nested message and enum types in `CreateFeedResponse`.
pub mod create_feed_response {
    /// CreateFeedResponse payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// The created feed
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::Feed>,
        /// AlreadyCreated indicates if the feed already existed (the create is idempotent)
        #[prost(bool, tag = "2")]
        pub already_created: bool,
    }
}
// ---------------------------------------

/// DeleteFeedRequest is used to delete a feed from a given twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFeedRequest {
    /// DeleteFeedRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// DeleteFeedRequest mandatory arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<delete_feed_request::Arguments>,
}
/// Nested message and enum types in `DeleteFeedRequest`.
pub mod delete_feed_request {
    /// DeleteFeedRequest arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// Feed to delete
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::Feed>,
    }
}
/// DeleteFeedResponse describes a deleted feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFeedResponse {
    /// DeleteFeedResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// DeleteFeedResponse payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<delete_feed_response::Payload>,
}
/// Nested message and enum types in `DeleteFeedResponse`.
pub mod delete_feed_response {
    /// DeleteFeedResponse payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::Feed>,
    }
}
// ---------------------------------------

/// UpdateFeedRequest is used to update attributes (including metadata) of a given feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeedRequest {
    /// UpdateFeedRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// UpdateFeedRequest arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<update_feed_request::Arguments>,
    /// UpdateFeedRequest payload
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<update_feed_request::Payload>,
}
/// Nested message and enum types in `UpdateFeedRequest`.
pub mod update_feed_request {
    /// UpdateFeedRequest payload. One or more fields can be provided, depending on what needs to be updated.
    /// Note that the specified metadata changes are applied in the following order:
    /// tags, values, labels, comments
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// storeLast dictates whether to store the last shared sample of a feed.
        #[prost(message, optional, tag = "1")]
        pub store_last: ::core::option::Option<bool>,
        /// tags are the set of tags to add or remove.
        #[prost(message, optional, tag = "2")]
        pub tags: ::core::option::Option<super::Tags>,
        /// values are descriptive individual data items to add/remove.
        #[prost(message, optional, tag = "3")]
        pub values: ::core::option::Option<super::Values>,
        /// labels are human-readable set of labels (language-specific) to add or remove.
        #[prost(message, optional, tag = "4")]
        pub labels: ::core::option::Option<super::LabelUpdate>,
        /// comments are the human-readable extended descriptions (language-specific) to add or remove.
        #[prost(message, optional, tag = "5")]
        pub comments: ::core::option::Option<super::CommentUpdate>,
    }
    /// UpdateFeedRequest arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::Feed>,
    }
}
/// UpdateFeedResponse is used to indicate a successful update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeedResponse {
    /// UpdateFeedResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    ///UpdateFeedResponse payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<update_feed_response::Payload>,
}
/// Nested message and enum types in `UpdateFeedResponse`.
pub mod update_feed_response {
    /// UpdateFeedResponse payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// Updated Twin
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::Feed>,
    }
}
// ---------------------------------------

/// ShareFeedDataRequest is used to share a new sample of data for the given feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareFeedDataRequest {
    /// ShareFeedDataRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// ShareFeedDataRequest arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<share_feed_data_request::Arguments>,
    /// ShareFeedDataRequest payload
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<share_feed_data_request::Payload>,
}
/// Nested message and enum types in `ShareFeedDataRequest`.
pub mod share_feed_data_request {
    /// ShareFeedDataRequest payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(message, optional, tag = "1")]
        pub sample: ::core::option::Option<super::FeedData>,
    }
    /// ShareFeedDataRequest arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::Feed>,
    }
}
/// ShareFeedDataResponse is used to indicate a successful feed share.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareFeedDataResponse {
    /// ShareFeedDataResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
}
// ---------------------------------------

/// ListAllFeedsRequest is used to list all the feeds owned by a given twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllFeedsRequest {
    /// ListAllFeedsRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// ListAllFeedsRequest arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<list_all_feeds_request::Arguments>,
    /// Limit the results according to the value
    /// (optional: when not supplied, assume no default limits required - See <https://ioticlabs.atlassian.net/browse/FO-1362>)
    #[prost(message, optional, tag = "3")]
    pub range: ::core::option::Option<Range>,
}
/// Nested message and enum types in `ListAllFeedsRequest`.
pub mod list_all_feeds_request {
    /// ListAllFeedsRequest mandatory arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// Identifier of the twin owning this feed
        #[prost(message, optional, tag = "1")]
        pub twin_id: ::core::option::Option<super::TwinId>,
    }
}
/// ListAllFeedsResponse describes the list of the feeds owned by a twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllFeedsResponse {
    /// ListAllFeedsResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// ListAllFeedsResponse payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<list_all_feeds_response::Payload>,
}
/// Nested message and enum types in `ListAllFeedsResponse`.
pub mod list_all_feeds_response {
    /// ListAllFeedsResponse payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// List of the feeds owned by the twin
        #[prost(message, repeated, tag = "1")]
        pub feeds: ::prost::alloc::vec::Vec<super::Feed>,
    }
}
/// Description of twin: Provides public metadata lookup for individual resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeFeedRequest {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// Language code for labels and comments. If set, only the label and comment in the given language will be returned
    /// instead of all. This field does not apply to values and tags which are always returned in full.
    #[prost(message, optional, tag = "2")]
    pub lang: ::core::option::Option<::prost::alloc::string::String>,
    /// DescribeFeedRequest mandatory arguments
    #[prost(message, optional, tag = "3")]
    pub args: ::core::option::Option<describe_feed_request::Arguments>,
}
/// Nested message and enum types in `DescribeFeedRequest`.
pub mod describe_feed_request {
    /// Only one action argument is necessary.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// Feed to describe
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::Feed>,
        /// optional HostID to describe a remote feed
        #[prost(message, optional, tag = "2")]
        pub remote_host_id: ::core::option::Option<super::HostId>,
    }
}
/// Describe feed response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeFeedResponse {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<describe_feed_response::Payload>,
}
/// Nested message and enum types in `DescribeFeedResponse`.
pub mod describe_feed_response {
    /// Metadata result databag.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetaResult {
        /// Labels in all languages set for the feed. (Or: Only label in chosen language, if lang field was specified in the
        /// request.)
        #[prost(message, repeated, tag = "1")]
        pub labels: ::prost::alloc::vec::Vec<super::LangLiteral>,
        /// values semantically describing the share payload of Feed or expected arguments for a Control request
        #[prost(message, repeated, tag = "2")]
        pub values: ::prost::alloc::vec::Vec<super::Value>,
        #[prost(string, repeated, tag = "3")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Comments in all languages set for the feed. (Or: Only comment in chosen language, if lang field was specified in
        /// the request.)
        #[prost(message, repeated, tag = "4")]
        pub comments: ::prost::alloc::vec::Vec<super::LangLiteral>,
        /// Whether this feed might have its most recent data sample stored. If so, it can be retrieved via FetchLastStored
        /// request. (See interest API)
        #[prost(bool, tag = "5")]
        pub store_last: bool,
    }
    /// DescribeFeedResponse payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::Feed>,
        #[prost(message, optional, tag = "2")]
        pub result: ::core::option::Option<MetaResult>,
        #[prost(message, optional, tag = "3")]
        pub remote_host_id: ::core::option::Option<super::HostId>,
    }
}
/// UpsertFeedWithMeta is used to describe the full feed state. Used in UpsertTwinRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpsertFeedWithMeta {
    /// Id of the feed to create/update
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Labels are human-readable set of labels (language-specific) to set
    #[prost(message, repeated, tag = "2")]
    pub labels: ::prost::alloc::vec::Vec<LangLiteral>,
    /// Comments are human-readable set of labels (language-specific) to set
    #[prost(message, repeated, tag = "3")]
    pub comments: ::prost::alloc::vec::Vec<LangLiteral>,
    /// storeLast dictates whether to store the last shared sample of the feed. Default 'False'
    #[prost(bool, tag = "4")]
    pub store_last: bool,
    /// values to set
    #[prost(message, repeated, tag = "5")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[doc = r" Generated client implementations."]
pub mod feed_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Create a feed owned by the provided twin."]
    #[doc = " A twin may have one or more feeds. Any twin can subscribe to a feed (access control permitting)."]
    #[doc = " A feed generates data in a 1-to-many relationship: one feed may produce data that is used by many consumers (twins)."]
    #[derive(Debug, Clone)]
    pub struct FeedApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FeedApiClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FeedApiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FeedApiClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            FeedApiClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Creates a feed owned by a twin. (Idempotent)"]
        pub async fn create_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFeedRequest>,
        ) -> Result<tonic::Response<super::CreateFeedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.FeedAPI/CreateFeed");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a feed owned by a twin. (Idempotent)"]
        pub async fn delete_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFeedRequest>,
        ) -> Result<tonic::Response<super::DeleteFeedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.FeedAPI/DeleteFeed");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates attributes of a feed, including its metadata."]
        pub async fn update_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFeedRequest>,
        ) -> Result<tonic::Response<super::UpdateFeedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.FeedAPI/UpdateFeed");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Shares a new sample of data for the given feed which any (interest) subscribers can receive."]
        pub async fn share_feed_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ShareFeedDataRequest>,
        ) -> Result<tonic::Response<super::ShareFeedDataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.FeedAPI/ShareFeedData");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List all feeds owned by a twin."]
        pub async fn list_all_feeds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAllFeedsRequest>,
        ) -> Result<tonic::Response<super::ListAllFeedsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.FeedAPI/ListAllFeeds");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Describe feed."]
        pub async fn describe_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeFeedRequest>,
        ) -> Result<tonic::Response<super::DescribeFeedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.FeedAPI/DescribeFeed");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// SearchRequest describes a search request used for both synchronous and asynchronous search.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRequest {
    /// Search request headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// Search request scope
    #[prost(enumeration = "Scope", tag = "2")]
    pub scope: i32,
    /// Search request lang. It implies both search and result text language
    #[prost(message, optional, tag = "3")]
    pub lang: ::core::option::Option<::prost::alloc::string::String>,
    /// Search request payload
    #[prost(message, optional, tag = "6")]
    pub payload: ::core::option::Option<search_request::Payload>,
    /// Search request range
    #[prost(message, optional, tag = "20")]
    pub range: ::core::option::Option<Range>,
}
/// Nested message and enum types in `SearchRequest`.
pub mod search_request {
    /// Search request payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// Expected response type
        #[prost(enumeration = "super::ResponseType", tag = "1")]
        pub response_type: i32,
        /// UTC time (millis from epoch / unix time) when this search request has to be considered expired.
        #[prost(message, optional, tag = "2")]
        pub expiry_timeout: ::core::option::Option<::prost_types::Timestamp>,
        /// Search Request filters
        #[prost(message, optional, tag = "3")]
        pub filter: ::core::option::Option<payload::Filter>,
    }
    /// Nested message and enum types in `Payload`.
    pub mod payload {
        /// Search request filters, any of these can be used in combination or on their own.
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Filter {
            /// Text filtering. One or more keywords which must match either text from twin/feed labels/comments (in the given
            /// language). Note that any (rather than all) of the keywords will produce a match.
            #[prost(message, optional, tag = "1")]
            pub text: ::core::option::Option<::prost::alloc::string::String>,
            /// Location filtering: area within which twins must be located
            #[prost(message, optional, tag = "2")]
            pub location: ::core::option::Option<super::super::GeoCircle>,
            /// Properties filtering: one or more exact properties, all of which twins must have.
            #[prost(message, repeated, tag = "3")]
            pub properties: ::prost::alloc::vec::Vec<super::super::Property>,
        }
    }
}
// ---------------------------------------------------------------------------------------------------------------------

/// SearchResponse describes a result associated to a search request.
/// It contains all the matching twins/feeds according to the request scope/range/lang/filters in the expected response type format.
/// In the decentralised iotics operating environment, each node in the network generates a response and the client is expected to
/// receive a stream of response messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// Search response payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<search_response::Payload>,
}
/// Nested message and enum types in `SearchResponse`.
pub mod search_response {
    /// Search response feed details. Included with response type: FULL.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeedDetails {
        /// Feed
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::Feed>,
        /// The feed human readable label in the language specified in the request (if set)
        #[prost(string, tag = "2")]
        pub label: ::prost::alloc::string::String,
        /// whether offers the ability to store last received value
        #[prost(bool, tag = "3")]
        pub store_last: bool,
    }
    /// Search response twin details.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TwinDetails {
        /// Twin identifier. Included with response type: FULL, LOCATED and MINIMAL
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::TwinId>,
        /// Twin location (if set). Included with response type: FULL and LOCATED
        #[prost(message, optional, tag = "2")]
        pub location: ::core::option::Option<super::GeoLocation>,
        /// Twin human readable label in the language specified in the request (if set). Included with response type: FULL and LOCATED
        #[prost(string, tag = "3")]
        pub label: ::prost::alloc::string::String,
        /// Twin tags. Included with response type: FULL
        #[prost(string, repeated, tag = "4")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Twin custom properties. Does not include labels/comments/location. Included with response type: FULL
        #[prost(message, repeated, tag = "5")]
        pub properties: ::prost::alloc::vec::Vec<super::Property>,
        /// Feed details. Included with response type: FULL
        #[prost(message, repeated, tag = "10")]
        pub feeds: ::prost::alloc::vec::Vec<FeedDetails>,
    }
    /// Search Response Payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// Type of the response.
        #[prost(enumeration = "super::ResponseType", tag = "1")]
        pub response_type: i32,
        /// Response status - if present indicates that this response is invalid
        #[prost(message, optional, tag = "2")]
        pub status: ::core::option::Option<super::super::super::google::rpc::Status>,
        /// Response host identifier - indicates from which host this response comes from
        #[prost(message, optional, tag = "4")]
        pub remote_host_id: ::core::option::Option<super::HostId>,
        /// Matching twins
        #[prost(message, repeated, tag = "10")]
        pub twins: ::prost::alloc::vec::Vec<TwinDetails>,
    }
}
// ---------------------------------------------------------------------------------------------------------------------

/// Dispatch Search Response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DispatchSearchResponse {}
/// ResponseType describes the type of the search response.
/// - FULL - Returns full responses including twins and feeds identifiers, labels/comments (for all languages if no language provided), properties and location
/// - LOCATED - Returns located responses including twins identifier, location and label (for the provided language or default)
/// - MINIMAL - Returns minimal responses including twins identifier only
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResponseType {
    Full = 0,
    Located = 1,
    Minimal = 2,
}
#[doc = r" Generated client implementations."]
pub mod search_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " SearchAPI provides a set of services to run synchronous and asynchronous search."]
    #[derive(Debug, Clone)]
    pub struct SearchApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SearchApiClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SearchApiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SearchApiClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SearchApiClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Send a search request. Results are expected asynchronously."]
        pub async fn dispatch_search_request(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRequest>,
        ) -> Result<tonic::Response<super::DispatchSearchResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/iotics.api.SearchAPI/DispatchSearchRequest");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Run a synchronous search based on a user timeout."]
        pub async fn synchronous_search(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SearchResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/iotics.api.SearchAPI/SynchronousSearch");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Receive all search responses associated to a set of Search request for a given client application ID."]
        pub async fn receive_all_search_responses(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscriptionHeaders>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SearchResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/iotics.api.SearchAPI/ReceiveAllSearchResponses",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
/// Twin is the virtual representation of a (physical, purely virtual or hybrid) device,
/// is only ever located in the host it was created.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Twin {
    /// Unique ID of the twin, assigned by the user.
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<TwinId>,
    /// Visibility of this twin
    #[prost(enumeration = "Visibility", tag = "2")]
    pub visibility: i32,
}
/// List all twins.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllTwinsRequest {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// Limit the results according to the value (optional: when not supplied, assume no default limits required - platform specific).
    #[prost(message, optional, tag = "20")]
    pub range: ::core::option::Option<Range>,
}
/// Response of the list all twins request.
/// Note this is useful for sync responses. In case there are too many twins (millions)
/// this request may fail. Better opt for async behaviour via stomp/websocket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllTwinsResponse {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<list_all_twins_response::Payload>,
}
/// Nested message and enum types in `ListAllTwinsResponse`.
pub mod list_all_twins_response {
    /// Payload of listed twins.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(message, repeated, tag = "1")]
        pub twins: ::prost::alloc::vec::Vec<super::Twin>,
    }
}
/// Message returned by the List service as a stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTwinsResponse {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<list_twins_response::Payload>,
}
/// Nested message and enum types in `ListTwinsResponse`.
pub mod list_twins_response {
    /// Payload of listed twins.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// the twin - may be omitted if the response code is not a success code
        #[prost(message, optional, tag = "1")]
        pub twin: ::core::option::Option<super::Twin>,
    }
}
/// CreateTwinRequest is made to create a twin (idempotent).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTwinRequest {
    /// Common headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// Request-specific payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<create_twin_request::Payload>,
}
/// Nested message and enum types in `CreateTwinRequest`.
pub mod create_twin_request {
    /// Arguments identifies the twin to create.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// Unique ID of the twin to create
        #[prost(message, optional, tag = "1")]
        pub twin_id: ::core::option::Option<super::TwinId>,
    }
}
/// CreateTwinResponse is received when a twin has been created.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTwinResponse {
    /// Common headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// Request-specific payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<create_twin_response::Payload>,
}
/// Nested message and enum types in `CreateTwinResponse`.
pub mod create_twin_response {
    /// Payload identifies the twin which was created.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// created twin
        #[prost(message, optional, tag = "1")]
        pub twin: ::core::option::Option<super::Twin>,
        /// whether the twin exists already (creating an existing twin is idempotent). Optional, with default=false.
        #[prost(bool, tag = "2")]
        pub already_created: bool,
    }
}
// ---------------------------------------

/// DeleteRequest is made to delete a particular twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTwinRequest {
    /// Common headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// Request-specific arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<delete_twin_request::Arguments>,
}
/// Nested message and enum types in `DeleteTwinRequest`.
pub mod delete_twin_request {
    /// Arguments identifies the twin to delete.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// Unique ID of the twin to delete
        #[prost(message, optional, tag = "1")]
        pub twin_id: ::core::option::Option<super::TwinId>,
    }
}
/// Deleted is received when a twin has been deleted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTwinResponse {
    /// Common headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// Request-specific response
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<delete_twin_response::Payload>,
}
/// Nested message and enum types in `DeleteTwinResponse`.
pub mod delete_twin_response {
    /// Payload identifies the twin which was deleted.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// Details of twin which was deleted
        #[prost(message, optional, tag = "1")]
        pub twin: ::core::option::Option<super::Twin>,
    }
}
/// Description of twin: Provides public metadata lookup for individual resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeTwinRequest {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// Language code for labels and comments. If set, only the label and comment in the given language will be returned
    /// instead of all. This field does not apply to tags and properties which are always returend in full.
    #[prost(message, optional, tag = "2")]
    pub lang: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub args: ::core::option::Option<describe_twin_request::Arguments>,
}
/// Nested message and enum types in `DescribeTwinRequest`.
pub mod describe_twin_request {
    /// Only one action argument is necessary.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// unique id of the twin to describe
        #[prost(message, optional, tag = "1")]
        pub twin_id: ::core::option::Option<super::TwinId>,
        /// optional HostID to describe a remote twin
        #[prost(message, optional, tag = "2")]
        pub remote_host_id: ::core::option::Option<super::HostId>,
    }
}
/// Metadata message for this Feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMeta {
    #[prost(message, optional, tag = "1")]
    pub feed_id: ::core::option::Option<FeedId>,
    #[prost(message, repeated, tag = "2")]
    pub labels: ::prost::alloc::vec::Vec<LangLiteral>,
    #[prost(bool, tag = "3")]
    pub store_last: bool,
}
/// The response for a description request on this twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeTwinResponse {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<describe_twin_response::Payload>,
}
/// Nested message and enum types in `DescribeTwinResponse`.
pub mod describe_twin_response {
    /// Metadata result data bag for this feed.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetaResult {
        #[prost(message, optional, tag = "1")]
        pub location: ::core::option::Option<super::GeoLocation>,
        /// Labels in all languages set for the twin. (Or: Only label in chosen language, if lang field was specified in
        /// the request.)
        #[prost(message, repeated, tag = "2")]
        pub labels: ::prost::alloc::vec::Vec<super::LangLiteral>,
        /// Comments in all languages set for the twin. (Or: Only comment in chosen language, if lang field was specified
        /// in the request.)
        #[prost(message, repeated, tag = "3")]
        pub comments: ::prost::alloc::vec::Vec<super::LangLiteral>,
        #[prost(message, repeated, tag = "4")]
        pub feeds: ::prost::alloc::vec::Vec<super::FeedMeta>,
        #[prost(string, repeated, tag = "5")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Custom properties associated with this twin. Does not include labels/comments/location.
        #[prost(message, repeated, tag = "6")]
        pub properties: ::prost::alloc::vec::Vec<super::Property>,
    }
    /// Payload of described twins.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// the twin
        #[prost(message, optional, tag = "1")]
        pub twin: ::core::option::Option<super::Twin>,
        /// the description details
        #[prost(message, optional, tag = "2")]
        pub result: ::core::option::Option<MetaResult>,
        /// optional - if present indicates this response comes from a remote host
        #[prost(message, optional, tag = "3")]
        pub remote_host_id: ::core::option::Option<super::HostId>,
    }
}
// ---------------------------------------

/// PropertyUpdate describes the update of a twin properties.
/// Can be used to add, replace, or delete properties. The order of operations is:
/// clearedAll (if True), deleted, deletedByKey, added.
/// Note that internal properties (such as location) cannot be modified here.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyUpdate {
    /// Delete all properties currently set on the twin.
    #[prost(bool, tag = "1")]
    pub cleared_all: bool,
    /// Delete specific exact properties (by key and value). This operation is ignored if clearAll is True.
    #[prost(message, repeated, tag = "2")]
    pub deleted: ::prost::alloc::vec::Vec<Property>,
    /// Delete any properties with the given keys (predicates). This operation is ignored if clearAll is True.
    #[prost(string, repeated, tag = "3")]
    pub deleted_by_key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Adds the given properties
    #[prost(message, repeated, tag = "4")]
    pub added: ::prost::alloc::vec::Vec<Property>,
}
/// VisibilityUpdate describes the update of a twin visibility.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisibilityUpdate {
    /// New visibility for this twin
    #[prost(enumeration = "Visibility", tag = "1")]
    pub visibility: i32,
}
/// GeoLocationUpdate describes the update of a twin location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoLocationUpdate {
    /// New location of the twin. If unset, the previously set location will be removed
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<GeoLocation>,
}
/// UpdateTwinRequest is used to update attributes (including metadata) of a given twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTwinRequest {
    /// UpdateTwinRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// UpdateTwinRequest arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<update_twin_request::Arguments>,
    /// UpdateTwinRequest payload
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<update_twin_request::Payload>,
}
/// Nested message and enum types in `UpdateTwinRequest`.
pub mod update_twin_request {
    /// UpdateTwinRequest mandatory arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// Unique ID of the twin to update
        #[prost(message, optional, tag = "1")]
        pub twin_id: ::core::option::Option<super::TwinId>,
    }
    /// UpdateTwinRequest payload. One or more fields can be provided, depending on what needs to be updated.
    /// Note that the specified metadata changes are applied in the following order:
    /// tags, visibility, properties, labels, comments, location
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// Tags are the set of tags to add or remove.
        #[prost(message, optional, tag = "1")]
        pub tags: ::core::option::Option<super::Tags>,
        /// New visibility
        #[prost(message, optional, tag = "2")]
        pub new_visibility: ::core::option::Option<super::VisibilityUpdate>,
        /// Custom properties to add/remove. Internal properties (such as location) cannot be modified here.
        #[prost(message, optional, tag = "3")]
        pub properties: ::core::option::Option<super::PropertyUpdate>,
        /// Labels are human-readable set of labels (language-specific) to add or remove.
        #[prost(message, optional, tag = "4")]
        pub labels: ::core::option::Option<super::LabelUpdate>,
        /// Comments are the human-readable extended descriptions (language-specific) to add or remove.
        #[prost(message, optional, tag = "5")]
        pub comments: ::core::option::Option<super::CommentUpdate>,
        /// Location to be set/unset
        #[prost(message, optional, tag = "6")]
        pub location: ::core::option::Option<super::GeoLocationUpdate>,
    }
}
/// UpdateTwinResponse describes an updated twin. It is received when the update operation is successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTwinResponse {
    /// UpdateTwinResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    ///UpdateTwinResponse payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<update_twin_response::Payload>,
}
/// Nested message and enum types in `UpdateTwinResponse`.
pub mod update_twin_response {
    /// UpdateTwinResponse payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// Updated Twin
        #[prost(message, optional, tag = "1")]
        pub twin: ::core::option::Option<super::Twin>,
    }
}
/// UpsertTwinRequest describes the full state of a twin + its feeds to create or update (full update)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpsertTwinRequest {
    /// UpdateTwinRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// UpdateTwinRequest payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<upsert_twin_request::Payload>,
}
/// Nested message and enum types in `UpsertTwinRequest`.
pub mod upsert_twin_request {
    /// UpsertTwinRequest payload. This state will be applied to the twin/feeds
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// Unique ID of the twin to create/update
        #[prost(string, tag = "1")]
        pub twin_id: ::prost::alloc::string::String,
        /// Twin visibility. Default value: 'PRIVATE'
        #[prost(enumeration = "super::Visibility", tag = "2")]
        pub visibility: i32,
        /// Labels are human-readable set of labels (language-specific) to set
        #[prost(message, repeated, tag = "3")]
        pub labels: ::prost::alloc::vec::Vec<super::LangLiteral>,
        /// Comments are human-readable set of labels (language-specific) to set
        #[prost(message, repeated, tag = "4")]
        pub comments: ::prost::alloc::vec::Vec<super::LangLiteral>,
        /// Twin Properties to set
        #[prost(message, repeated, tag = "5")]
        pub properties: ::prost::alloc::vec::Vec<super::Property>,
        /// Twin location to set. If not set the Twin will have no location
        #[prost(message, optional, tag = "6")]
        pub location: ::core::option::Option<super::GeoLocation>,
        /// Feeds with metadata to set to the twin
        #[prost(message, repeated, tag = "7")]
        pub feeds: ::prost::alloc::vec::Vec<super::UpsertFeedWithMeta>,
    }
}
/// UpsertTwinResponse is received when a twin and its feeds have been created/updated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpsertTwinResponse {
    /// Common headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// Request-specific payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<upsert_twin_response::Payload>,
}
/// Nested message and enum types in `UpsertTwinResponse`.
pub mod upsert_twin_response {
    /// Payload identifies the twin which was created.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// created/updated twin
        #[prost(string, tag = "1")]
        pub twin_id: ::prost::alloc::string::String,
    }
}
#[doc = r" Generated client implementations."]
pub mod twin_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " TwinAPI enables creation and management of Iotics twins."]
    #[derive(Debug, Clone)]
    pub struct TwinApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TwinApiClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TwinApiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TwinApiClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TwinApiClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " CreateTwin creates a twin."]
        pub async fn create_twin(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTwinRequest>,
        ) -> Result<tonic::Response<super::CreateTwinResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.TwinAPI/CreateTwin");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UpsertTwin creates or update a twin with its metadata + the twin feeds with their metadata."]
        #[doc = " The full state is applied (ie. if the operation succeeds the state of the twin/feeds will be the one"]
        #[doc = " described in the payload)"]
        pub async fn upsert_twin(
            &mut self,
            request: impl tonic::IntoRequest<super::UpsertTwinRequest>,
        ) -> Result<tonic::Response<super::UpsertTwinResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.TwinAPI/UpsertTwin");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DeleteTwin deletes a twin."]
        pub async fn delete_twin(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTwinRequest>,
        ) -> Result<tonic::Response<super::DeleteTwinResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.TwinAPI/DeleteTwin");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UpdateTwin updates a twin (partial update)."]
        pub async fn update_twin(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTwinRequest>,
        ) -> Result<tonic::Response<super::UpdateTwinResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.TwinAPI/UpdateTwin");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Describes a twin."]
        pub async fn describe_twin(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeTwinRequest>,
        ) -> Result<tonic::Response<super::DescribeTwinResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.TwinAPI/DescribeTwin");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List all twins."]
        pub async fn list_all_twins(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAllTwinsRequest>,
        ) -> Result<tonic::Response<super::ListAllTwinsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/iotics.api.TwinAPI/ListAllTwins");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// An interest is the relationship between a Twin and a Feed. For example, creating
/// an interest on (following) a Feed results in any data shared on said Feed to be forwarded to
/// the associated Twin. Interests can be revoked either by the subscriber or provider and there
/// may only be one interest between any unique Twin and Feed combination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interest {
    /// Follower twin unique identifier.
    #[prost(message, optional, tag = "2")]
    pub follower_twin_id: ::core::option::Option<TwinId>,
    /// a reference to the interested feed
    #[prost(message, optional, tag = "3")]
    pub followed_feed: ::core::option::Option<interest::FollowedFeed>,
}
/// Nested message and enum types in `Interest`.
pub mod interest {
    /// FollowedFeed fully identify the (local or remote) feed to follow.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FollowedFeed {
        /// Followed feed identifier
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::Feed>,
        /// Feed remote host identifier (If not specified, the Interest is taken to be in scope of the host from which a request is made.)
        #[prost(message, optional, tag = "2")]
        pub host_id: ::core::option::Option<super::HostId>,
    }
}
/// CreateInterestRequest is used to create an interest between a twin and a feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInterestRequest {
    /// CreateInterestRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// CreateInterestRequest arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<create_interest_request::Arguments>,
    /// CreateInterestRequest payload
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<create_interest_request::Payload>,
}
/// Nested message and enum types in `CreateInterestRequest`.
pub mod create_interest_request {
    /// CreateInterestRequest payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(message, optional, tag = "2")]
        pub interest: ::core::option::Option<super::Interest>,
    }
    /// CreateInterestRequest mandatory arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// Follower twin unique identifier
        #[prost(message, optional, tag = "1")]
        pub twin_id: ::core::option::Option<super::TwinId>,
    }
}
/// CreateInterestResponse describes a successfully created interest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInterestResponse {
    /// CreateInterestResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// CreateInterestResponse payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<create_interest_response::Payload>,
}
/// Nested message and enum types in `CreateInterestResponse`.
pub mod create_interest_response {
    /// CreateInterestResponse payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// Created interest
        #[prost(message, optional, tag = "2")]
        pub interest: ::core::option::Option<super::Interest>,
        /// whether the interest exists already (creating an existing interest is idempotent).
        #[prost(bool, tag = "3")]
        pub already_created: bool,
    }
}
// ---------------------------------------

/// ListAllInterestsRequest is used to list all interests initiated by a given twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllInterestsRequest {
    /// ListAllInterestsRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// ListAllInterestsRequest arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<list_all_interests_request::Arguments>,
    /// Limit the results according to the value (optional: when not supplied, assume no default limits required - platform specific)
    #[prost(message, optional, tag = "20")]
    pub range: ::core::option::Option<Range>,
}
/// Nested message and enum types in `ListAllInterestsRequest`.
pub mod list_all_interests_request {
    /// ListAllInterestsRequest mandatory arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// Follower twin unique identifier
        #[prost(message, optional, tag = "1")]
        pub twin_id: ::core::option::Option<super::TwinId>,
    }
}
/// ListAllInterestsResponse describes all the interest initiated by the given twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllInterestsResponse {
    /// ListAllInterestsResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// ListAllInterestsResponse payload.
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<list_all_interests_response::Payload>,
}
/// Nested message and enum types in `ListAllInterestsResponse`.
pub mod list_all_interests_response {
    /// ListAllInterestsResponse payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(message, repeated, tag = "1")]
        pub interests: ::prost::alloc::vec::Vec<super::Interest>,
    }
}
// ---------------------------------------

/// FetchInterestRequest is used to initiate a stream to get the feed data shared on a given interest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchInterestRequest {
    /// FetchInterestRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// FetchInterestRequest arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<fetch_interest_request::Arguments>,
    /// whether to fetch the last stored value if available (false by default)
    #[prost(message, optional, tag = "3")]
    pub fetch_last_stored: ::core::option::Option<bool>,
}
/// Nested message and enum types in `FetchInterestRequest`.
pub mod fetch_interest_request {
    /// FetchInterestRequest mandatory arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// The interest
        #[prost(message, optional, tag = "1")]
        pub interest: ::core::option::Option<super::Interest>,
    }
}
/// FetchInterestResponse describes a feed value shared on a given interest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchInterestResponse {
    /// FetchInterestResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// FetchInterestResponse payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<fetch_interest_response::Payload>,
}
/// Nested message and enum types in `FetchInterestResponse`.
pub mod fetch_interest_response {
    /// FetchInterestResponse payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// The interest
        #[prost(message, optional, tag = "1")]
        pub interest: ::core::option::Option<super::Interest>,
        /// The shared data
        #[prost(message, optional, tag = "2")]
        pub feed_data: ::core::option::Option<super::FeedData>,
    }
}
// ---------------------------------------

/// FetchLastStoredRequest is used to fetch the last stored value on a given interest if available.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchLastStoredRequest {
    /// FetchLastStoredRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// FetchLastStoredRequest arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<fetch_last_stored_request::Arguments>,
}
/// Nested message and enum types in `FetchLastStoredRequest`.
pub mod fetch_last_stored_request {
    /// FetchLastStoredRequest mandatory arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// The interest
        #[prost(message, optional, tag = "1")]
        pub interest: ::core::option::Option<super::Interest>,
    }
}
// ---------------------------------------

/// DeleteInterestRequest is used to delete an interest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInterestRequest {
    /// DeleteInterestRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// DeleteInterestRequest args
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<delete_interest_request::Arguments>,
}
/// Nested message and enum types in `DeleteInterestRequest`.
pub mod delete_interest_request {
    /// DeleteInterestRequest mandatory arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// The interest
        #[prost(message, optional, tag = "1")]
        pub interest: ::core::option::Option<super::Interest>,
    }
}
/// DeleteInterestResponse describes a deleted interest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInterestResponse {
    /// DeleteInterestResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<Headers>,
    /// DeleteInterestResponse payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<delete_interest_response::Payload>,
}
/// Nested message and enum types in `DeleteInterestResponse`.
pub mod delete_interest_response {
    /// DeleteInterestResponse payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// The deleted interest.
        #[prost(message, optional, tag = "1")]
        pub interest: ::core::option::Option<super::Interest>,
    }
}
#[doc = r" Generated client implementations."]
pub mod interest_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " InterestAPI enables creation and management of interests between a twin and a feed."]
    #[derive(Debug, Clone)]
    pub struct InterestApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InterestApiClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InterestApiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterestApiClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            InterestApiClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Fetch feed data for this interest."]
        pub async fn fetch_interests(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchInterestRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::FetchInterestResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/iotics.api.InterestAPI/FetchInterests");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Fetch last data shared on this interest."]
        pub async fn fetch_last_stored(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchLastStoredRequest>,
        ) -> Result<tonic::Response<super::FetchInterestResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/iotics.api.InterestAPI/FetchLastStored");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List all interests associated to a given follower twin (Not implemented yet)."]
        pub async fn list_all_interests(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAllInterestsRequest>,
        ) -> Result<tonic::Response<super::ListAllInterestsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/iotics.api.InterestAPI/ListAllInterests");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create an interest between a follower twin and a followed feed."]
        pub async fn create_interest(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInterestRequest>,
        ) -> Result<tonic::Response<super::CreateInterestResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/iotics.api.InterestAPI/CreateInterest");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete an existing interest."]
        pub async fn delete_interest(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInterestRequest>,
        ) -> Result<tonic::Response<super::DeleteInterestResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/iotics.api.InterestAPI/DeleteInterest");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
