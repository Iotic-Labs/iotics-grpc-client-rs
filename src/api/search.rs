/// SearchRequest describes a search request used for both synchronous and asynchronous search.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRequest {
    /// Search request headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
    /// Search request scope
    #[prost(enumeration = "super::common::Scope", tag = "2")]
    pub scope: i32,
    /// Search request lang. It implies both search and result text language
    #[prost(message, optional, tag = "3")]
    pub lang: ::core::option::Option<::prost::alloc::string::String>,
    /// Search request payload
    #[prost(message, optional, tag = "6")]
    pub payload: ::core::option::Option<search_request::Payload>,
    /// Search request range
    #[prost(message, optional, tag = "20")]
    pub range: ::core::option::Option<super::common::Range>,
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
        #[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Filter {
            /// Text filtering. One or more keywords which must match either text from twin/feed labels/comments (in the given
            /// language). Note that any (rather than all) of the keywords will produce a match.
            #[prost(message, optional, tag = "1")]
            pub text: ::core::option::Option<::prost::alloc::string::String>,
            /// Location filtering: area within which twins must be located
            #[prost(message, optional, tag = "2")]
            pub location: ::core::option::Option<super::super::super::common::GeoCircle>,
            /// Properties filtering: one or more exact properties, all of which twins must have.
            #[prost(message, repeated, tag = "3")]
            pub properties: ::prost::alloc::vec::Vec<super::super::super::common::Property>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
    /// Search response payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<search_response::Payload>,
}
/// Nested message and enum types in `SearchResponse`.
pub mod search_response {
    /// Search response feed details. Included with response type: FULL.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeedDetails {
        /// Feed
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::super::feed::Feed>,
        /// The feed human readable label in the language specified in the request (if set)
        #[prost(string, tag = "2")]
        pub label: ::prost::alloc::string::String,
        /// whether offers the ability to store last received value
        #[prost(bool, tag = "3")]
        pub store_last: bool,
    }
    /// Search response twin details.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TwinDetails {
        /// Twin identifier. Included with response type: FULL, LOCATED and MINIMAL
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::common::TwinId>,
        /// Twin location (if set). Included with response type: FULL and LOCATED
        #[prost(message, optional, tag = "2")]
        pub location: ::core::option::Option<super::super::common::GeoLocation>,
        /// Twin human readable label in the language specified in the request (if set). Included with response type: FULL and LOCATED
        #[prost(string, tag = "3")]
        pub label: ::prost::alloc::string::String,
        /// Twin tags. Included with response type: FULL
        #[prost(string, repeated, tag = "4")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Twin custom properties. Does not include labels/comments/location. Included with response type: FULL
        #[prost(message, repeated, tag = "5")]
        pub properties: ::prost::alloc::vec::Vec<super::super::common::Property>,
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
        pub status: ::core::option::Option<super::super::google::rpc::Status>,
        /// Response host identifier - indicates from which host this response comes from
        #[prost(message, optional, tag = "4")]
        pub remote_host_id: ::core::option::Option<super::super::common::HostId>,
        /// Matching twins
        #[prost(message, repeated, tag = "10")]
        pub twins: ::prost::alloc::vec::Vec<TwinDetails>,
    }
}
// ---------------------------------------------------------------------------------------------------------------------

/// Dispatch Search Response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DispatchSearchResponse {}
// ---------------------------------------------------------------------------------------------------------------------

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
        T::ResponseBody: Body + Send + Sync + 'static,
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
                http::uri::PathAndQuery::from_static("/search.SearchAPI/DispatchSearchRequest");
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
            let path = http::uri::PathAndQuery::from_static("/search.SearchAPI/SynchronousSearch");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Receive all search responses associated to a set of Search request for a given client application ID."]
        pub async fn receive_all_search_responses(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::SubscriptionHeaders>,
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
                http::uri::PathAndQuery::from_static("/search.SearchAPI/ReceiveAllSearchResponses");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
