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
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
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
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringLiteral {
    /// String representation of the value
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Literal is a metadata property type describing a literal with the given datatype (implicit datatype: rdfs:Literal).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Literal {
    /// XSD data type (e.g. double) without its namespace prefix (http://www.w3.org/2001/XMLSchema#). The following types
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
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uri {
    /// String representation of the value
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Property is a metadata property with a single value.
/// Multiple instances are used to represent a key (predicate) with multiple values.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
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
    #[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
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
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
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
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwinId {
    /// Twin Identifier (using DID format)
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// FeedID is a unique feed identifier (scoped to the TwinID).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedId {
    /// Feed Identifier string representation (simple string)
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Value is the definition of an individual piece of data within a Feed share. Values are purely descriptive, e.g. a
/// Feed follower should expect data to match the values associated with said Feed but must be able to recover where this
/// is not the case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// label is the unique identifier of the value. It is language-neutral. E.g.: "t" / "temp" / "temperature".
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    /// comment is the (optional) human-readable description of the value. It is language-specific. E.g.: "Engine oil temperature"
    #[prost(string, tag = "2")]
    pub comment: ::prost::alloc::string::String,
    /// unit is the (optional) fully qualified ontology string URI of the unit, e.g. http://purl.obolibrary.org/obo/UO_0000027
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
