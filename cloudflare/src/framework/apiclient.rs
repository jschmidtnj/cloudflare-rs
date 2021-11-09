//! This module contains the synchronous (blocking) API client.
use crate::framework::{
    endpoint::Endpoint,
    response::{ApiResponse, ApiResult},
};
use serde::Serialize;

/// Synchronously sends requests to the Cloudflare API.
pub trait ApiClient {
    /// Synchronously send a request to the Cloudflare API.
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn Endpoint<ResultType, QueryType, BodyType>,
    ) -> ApiResponse<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize;

    /// Synchronously send a text request to the Cloudflare API.
    fn request_text<QueryType, BodyType>(
        &self,
        endpoint: &dyn Endpoint<String, QueryType, BodyType>,
    ) -> ApiResponse<String>
        where
            QueryType: Serialize,
            BodyType: Serialize;
}
