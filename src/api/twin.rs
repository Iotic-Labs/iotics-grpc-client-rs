// ---------------------------------------------------------------------------------------------------------------------

/// Twin is the virtual representation of a (physical, purely virtual or hybrid) device,
/// is only ever located in the host it was created.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Twin {
    /// Unique ID of the twin, assigned by the user.
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::common::TwinId>,
    /// Visibility of this twin
    #[prost(enumeration = "super::common::Visibility", tag = "2")]
    pub visibility: i32,
}
/// List all twins.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllTwinsRequest {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
    /// Limit the results according to the value (optional: when not supplied, assume no default limits required - platform specific).
    #[prost(message, optional, tag = "20")]
    pub range: ::core::option::Option<super::common::Range>,
}
/// Response of the list all twins request.
/// Note this is useful for sync responses. In case there are too many twins (millions)
/// this request may fail. Better opt for async behaviour via stomp/websocket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllTwinsResponse {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
        pub twin_id: ::core::option::Option<super::super::common::TwinId>,
    }
}
/// CreateTwinResponse is received when a twin has been created.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTwinResponse {
    /// Common headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
        pub twin_id: ::core::option::Option<super::super::common::TwinId>,
    }
}
/// Deleted is received when a twin has been deleted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTwinResponse {
    /// Common headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
        pub twin_id: ::core::option::Option<super::super::common::TwinId>,
        /// optional HostID to describe a remote twin
        #[prost(message, optional, tag = "2")]
        pub remote_host_id: ::core::option::Option<super::super::common::HostId>,
    }
}
/// Metadata message for this Feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMeta {
    #[prost(message, optional, tag = "1")]
    pub feed_id: ::core::option::Option<super::common::FeedId>,
    #[prost(message, repeated, tag = "2")]
    pub labels: ::prost::alloc::vec::Vec<super::common::LangLiteral>,
    #[prost(bool, tag = "3")]
    pub store_last: bool,
}
/// Metadata result data bag for this feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaResult {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<super::common::GeoLocation>,
    /// Labels in all languages set for the twin. (Or: Only label in chosen language, if lang field was specified in
    /// the request.)
    #[prost(message, repeated, tag = "2")]
    pub labels: ::prost::alloc::vec::Vec<super::common::LangLiteral>,
    /// Comments in all languages set for the twin. (Or: Only comment in chosen language, if lang field was specified
    /// in the request.)
    #[prost(message, repeated, tag = "3")]
    pub comments: ::prost::alloc::vec::Vec<super::common::LangLiteral>,
    #[prost(message, repeated, tag = "4")]
    pub feeds: ::prost::alloc::vec::Vec<FeedMeta>,
    #[prost(string, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Custom properties associated with this twin. Does not include labels/comments/location.
    #[prost(message, repeated, tag = "6")]
    pub properties: ::prost::alloc::vec::Vec<super::common::Property>,
}
/// The response for a description request on this twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeTwinResponse {
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<describe_twin_response::Payload>,
}
/// Nested message and enum types in `DescribeTwinResponse`.
pub mod describe_twin_response {
    /// Payload of described twins.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// the twin
        #[prost(message, optional, tag = "1")]
        pub twin: ::core::option::Option<super::Twin>,
        /// the description details
        #[prost(message, optional, tag = "2")]
        pub result: ::core::option::Option<super::MetaResult>,
        /// optional - if present indicates this response comes from a remote host
        #[prost(message, optional, tag = "3")]
        pub remote_host_id: ::core::option::Option<super::super::common::HostId>,
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
    pub deleted: ::prost::alloc::vec::Vec<super::common::Property>,
    /// Delete any properties with the given keys (predicates). This operation is ignored if clearAll is True.
    #[prost(string, repeated, tag = "3")]
    pub deleted_by_key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Adds the given properties
    #[prost(message, repeated, tag = "4")]
    pub added: ::prost::alloc::vec::Vec<super::common::Property>,
}
/// VisibilityUpdate describes the update of a twin visibility.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisibilityUpdate {
    /// New visibility for this twin
    #[prost(enumeration = "super::common::Visibility", tag = "1")]
    pub visibility: i32,
}
/// GeoLocationUpdate describes the update of a twin location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoLocationUpdate {
    /// New location of the twin. If unset, the previously set location will be removed
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<super::common::GeoLocation>,
}
/// UpdateTwinRequest is used to update attributes (including metadata) of a given twin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTwinRequest {
    /// UpdateTwinRequest headers
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<super::common::Headers>,
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
        pub twin_id: ::core::option::Option<super::super::common::TwinId>,
    }
    /// UpdateTwinRequest payload. One or more fields can be provided, depending on what needs to be updated.
    /// Note that the specified metadata changes are applied in the following order:
    /// tags, visibility, properties, labels, comments, location
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        /// Tags are the set of tags to add or remove.
        #[prost(message, optional, tag = "1")]
        pub tags: ::core::option::Option<super::super::common::Tags>,
        /// New visibility
        #[prost(message, optional, tag = "2")]
        pub new_visibility: ::core::option::Option<super::VisibilityUpdate>,
        /// Custom properties to add/remove. Internal properties (such as location) cannot be modified here.
        #[prost(message, optional, tag = "3")]
        pub properties: ::core::option::Option<super::PropertyUpdate>,
        /// Labels are human-readable set of labels (language-specific) to add or remove.
        #[prost(message, optional, tag = "4")]
        pub labels: ::core::option::Option<super::super::common::LabelUpdate>,
        /// Comments are the human-readable extended descriptions (language-specific) to add or remove.
        #[prost(message, optional, tag = "5")]
        pub comments: ::core::option::Option<super::super::common::CommentUpdate>,
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
    pub headers: ::core::option::Option<super::common::Headers>,
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
            let path = http::uri::PathAndQuery::from_static("/twin.TwinAPI/CreateTwin");
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
            let path = http::uri::PathAndQuery::from_static("/twin.TwinAPI/DeleteTwin");
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
            let path = http::uri::PathAndQuery::from_static("/twin.TwinAPI/UpdateTwin");
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
            let path = http::uri::PathAndQuery::from_static("/twin.TwinAPI/DescribeTwin");
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
            let path = http::uri::PathAndQuery::from_static("/twin.TwinAPI/ListAllTwins");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
