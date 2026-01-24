////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Error Response: contains both an error code and some kind of helpful message
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
use axum::{body::Body, http::StatusCode as SC, response::IntoResponse};

use http::response::Response;

pub struct ErrorResponse {
    status_code: SC,
    error_msg: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(self.status_code)
            .body(Body::from(self.error_msg))
            .unwrap()
    }
}

pub fn make_error(status_code: SC, error_msg: String) -> ErrorResponse {
    ErrorResponse {
        status_code,
        error_msg,
    }
}
