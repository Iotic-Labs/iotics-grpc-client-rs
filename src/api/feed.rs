/// A feed representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    /// feed identifier (unique within the scope of a twin identifier)
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::common::FeedId>,
    /// twin unique identifier (twin to which the feed belongs)
    #[prost(message, optional, tag = "2")]
    pub twin_id: ::core::option::Option<super::common::TwinId>,
}
/// CreateFeedRequestCreate is used to create a new feed in a given twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeedRequest {
    /// CreateFeedRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
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
        pub feed_id: ::core::option::Option<super::super::common::FeedId>,
        /// StoreLast indicates if the last received value should be stored of not
        #[prost(bool, tag = "2")]
        pub store_last: bool,
    }
    /// Arguments describes the mandatory arguments to identify the twin the feed belongs to.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// Identifier of the twin owning this feed
        #[prost(message, optional, tag = "1")]
        pub twin_id: ::core::option::Option<super::super::common::TwinId>,
    }
}
/// CreateFeedResponse describes a created feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeedResponse {
    /// CreateFeedResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
        pub tags: ::core::option::Option<super::super::common::Tags>,
        /// values are descriptive individual data items to add/remove.
        #[prost(message, optional, tag = "3")]
        pub values: ::core::option::Option<super::super::common::Values>,
        /// labels are human-readable set of labels (language-specific) to add or remove.
        #[prost(message, optional, tag = "4")]
        pub labels: ::core::option::Option<super::super::common::LabelUpdate>,
        /// comments are the human-readable extended descriptions (language-specific) to add or remove.
        #[prost(message, optional, tag = "5")]
        pub comments: ::core::option::Option<super::super::common::CommentUpdate>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
        pub sample: ::core::option::Option<super::super::common::FeedData>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
}
// ---------------------------------------

/// ListAllFeedsRequest is used to list all the feeds owned by a given twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllFeedsRequest {
    /// ListAllFeedsRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
    /// ListAllFeedsRequest arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<list_all_feeds_request::Arguments>,
    /// Limit the results according to the value
    /// (optional: when not supplied, assume no default limits required - See <https://ioticlabs.atlassian.net/browse/FO-1362>)
    #[prost(message, optional, tag = "3")]
    pub range: ::core::option::Option<super::common::Range>,
}
/// Nested message and enum types in `ListAllFeedsRequest`.
pub mod list_all_feeds_request {
    /// ListAllFeedsRequest mandatory arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// Identifier of the twin owning this feed
        #[prost(message, optional, tag = "1")]
        pub twin_id: ::core::option::Option<super::super::common::TwinId>,
    }
}
/// ListAllFeedsResponse describes the list of the feeds owned by a twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllFeedsResponse {
    /// ListAllFeedsResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
        pub remote_host_id: ::core::option::Option<super::super::common::HostId>,
    }
}
/// Metadata result databag.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaResult {
    /// Labels in all languages set for the feed. (Or: Only label in chosen language, if lang field was specified in the
    /// request.)
    #[prost(message, repeated, tag = "1")]
    pub labels: ::prost::alloc::vec::Vec<super::common::LangLiteral>,
    /// values semantically describing the share payload of Feed or expected arguments for a Control request
    #[prost(message, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<super::common::Value>,
    #[prost(string, repeated, tag = "3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Comments in all languages set for the feed. (Or: Only comment in chosen language, if lang field was specified in
    /// the request.)
    #[prost(message, repeated, tag = "4")]
    pub comments: ::prost::alloc::vec::Vec<super::common::LangLiteral>,
    /// Whether this feed might have its most recent data sample stored. If so, it can be retrieved via FetchLastStored
    /// request. (See interest API)
    #[prost(bool, tag = "5")]
    pub store_last: bool,
}
/// Describe feed response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeFeedResponse {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<describe_feed_response::Payload>,
}
/// Nested message and enum types in `DescribeFeedResponse`.
pub mod describe_feed_response {
    /// DescribeFeedResponse payload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(message, optional, tag = "1")]
        pub feed: ::core::option::Option<super::Feed>,
        #[prost(message, optional, tag = "2")]
        pub result: ::core::option::Option<super::MetaResult>,
        #[prost(message, optional, tag = "3")]
        pub remote_host_id: ::core::option::Option<super::super::common::HostId>,
    }
}
/// UpsertFeedWithMeta is used to describe the full feed state. Used in UpsertTwinRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpsertFeedWithMeta {
    /// Id of the feed to create/update
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Labels are human-readable set of labels (language-specific) to set
    #[prost(message, repeated, tag = "2")]
    pub labels: ::prost::alloc::vec::Vec<super::common::LangLiteral>,
    /// Comments are human-readable set of labels (language-specific) to set
    #[prost(message, repeated, tag = "3")]
    pub comments: ::prost::alloc::vec::Vec<super::common::LangLiteral>,
    /// storeLast dictates whether to store the last shared sample of the feed. Default 'False'
    #[prost(bool, tag = "4")]
    pub store_last: bool,
    /// values to set
    #[prost(message, repeated, tag = "5")]
    pub values: ::prost::alloc::vec::Vec<super::common::Value>,
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
            let path = http::uri::PathAndQuery::from_static("/feed.FeedAPI/CreateFeed");
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
            let path = http::uri::PathAndQuery::from_static("/feed.FeedAPI/DeleteFeed");
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
            let path = http::uri::PathAndQuery::from_static("/feed.FeedAPI/UpdateFeed");
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
            let path = http::uri::PathAndQuery::from_static("/feed.FeedAPI/ShareFeedData");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List all feeds  owned by a twin."]
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
            let path = http::uri::PathAndQuery::from_static("/feed.FeedAPI/ListAllFeeds");
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
            let path = http::uri::PathAndQuery::from_static("/feed.FeedAPI/DescribeFeed");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
