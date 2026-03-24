// Updated:  2026-03-27 by Constructor Tech
pub mod api;
pub mod body;
pub mod codec;
pub mod error;
pub mod multipart;
pub mod sse;
pub mod ws;

// @cpt-begin:cpt-cf-oagw-algo-domain-sdk-definition:p1:inst-sdk-2
// @cpt-begin:cpt-cf-oagw-algo-domain-sdk-definition:p1:inst-sdk-4
pub mod models;
// @cpt-end:cpt-cf-oagw-algo-domain-sdk-definition:p1:inst-sdk-4
// @cpt-end:cpt-cf-oagw-algo-domain-sdk-definition:p1:inst-sdk-2

// @cpt-begin:cpt-cf-oagw-algo-domain-sdk-definition:p1:inst-sdk-5
pub use models::{
    AuthConfig, BurstConfig, CorsConfig, CorsHttpMethod, CreateRouteRequest,
    CreateRouteRequestBuilder, CreateUpstreamRequest, CreateUpstreamRequestBuilder, Endpoint,
    GrpcMatch, HeadersConfig, HttpMatch, HttpMethod, ListQuery, MatchRules, PassthroughMode,
    PathSuffixMode, PluginBinding, PluginsConfig, RateLimitAlgorithm, RateLimitConfig,
    RateLimitScope, RateLimitStrategy, RequestHeaderRules, ResponseHeaderRules, Route, Scheme,
    Server, SharingMode, SustainedRate, UpdateRouteRequest, UpdateRouteRequestBuilder,
    UpdateUpstreamRequest, UpdateUpstreamRequestBuilder, Upstream, Window,
};

pub use api::ServiceGatewayClientV1;
pub use body::Body;
pub use codec::Json;
pub use error::StreamingError;
pub use multipart::{MultipartBody, MultipartError, Part};
pub use sse::{FromServerEvent, ServerEvent, ServerEventsResponse, ServerEventsStream};
#[cfg(feature = "axum")]
pub use ws::axum_adapter;
pub use ws::{
    FromWebSocketMessage, WebSocketCloseFrame, WebSocketMessage, WebSocketReceiver,
    WebSocketSender, WebSocketSink, WebSocketStream, WebSocketStreamReceiver,
};
// @cpt-end:cpt-cf-oagw-algo-domain-sdk-definition:p1:inst-sdk-5
// @cpt-begin:cpt-cf-oagw-algo-domain-sdk-definition:p1:inst-sdk-6
// SDK crate provides: ServiceGatewayClientV1 trait, model types, error types, Body abstraction.
// @cpt-end:cpt-cf-oagw-algo-domain-sdk-definition:p1:inst-sdk-6
