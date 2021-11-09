/// An interest is the relationship between a Twin and a Feed. For example, creating
/// an interest on (following) a Feed results in any data shared on said Feed to be forwarded to
/// the associated Twin. Interests can be revoked either by the subscriber or provider and there
/// may only be one interest between any unique Twin and Feed combination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interest {
    /// Follower twin unique identifier.
    #[prost(message, optional, tag = "2")]
    pub follower_twin_id: ::core::option::Option<super::common::TwinId>,
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
        pub feed: ::core::option::Option<super::super::feed::Feed>,
        /// Feed remote host identifier (If not specified, the Interest is taken to be in scope of the host from which a request is made.)
        #[prost(message, optional, tag = "2")]
        pub host_id: ::core::option::Option<super::super::common::HostId>,
    }
}
/// CreateInterestRequest is used to create an interest between a twin and a feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInterestRequest {
    /// CreateInterestRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
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
        pub twin_id: ::core::option::Option<super::super::common::TwinId>,
    }
}
/// CreateInterestResponse describes a successfully created interest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInterestResponse {
    /// CreateInterestResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
    /// ListAllInterestsRequest arguments
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<list_all_interests_request::Arguments>,
    /// Limit the results according to the value (optional: when not supplied, assume no default limits required - platform specific)
    #[prost(message, optional, tag = "20")]
    pub range: ::core::option::Option<super::common::Range>,
}
/// Nested message and enum types in `ListAllInterestsRequest`.
pub mod list_all_interests_request {
    /// ListAllInterestsRequest mandatory arguments.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Arguments {
        /// Follower twin unique identifier
        #[prost(message, optional, tag = "1")]
        pub twin_id: ::core::option::Option<super::super::common::TwinId>,
    }
}
/// ListAllInterestsResponse describes all the interest initiated by the given twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllInterestsResponse {
    /// ListAllInterestsResponse headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
        pub feed_data: ::core::option::Option<super::super::common::FeedData>,
    }
}
// ---------------------------------------

/// FetchLastStoredRequest is used to fetch the last stored value on a given interest if available.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchLastStoredRequest {
    /// FetchLastStoredRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
            let path = http::uri::PathAndQuery::from_static("/interest.InterestAPI/FetchInterests");
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
                http::uri::PathAndQuery::from_static("/interest.InterestAPI/FetchLastStored");
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
                http::uri::PathAndQuery::from_static("/interest.InterestAPI/ListAllInterests");
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
            let path = http::uri::PathAndQuery::from_static("/interest.InterestAPI/CreateInterest");
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
            let path = http::uri::PathAndQuery::from_static("/interest.InterestAPI/DeleteInterest");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
