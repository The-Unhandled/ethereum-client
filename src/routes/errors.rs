use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Internal server error: {0}")]
    InternalError(String),

    //#[error("Bad request: {0}")]
    //BadRequest(String),

    // Add additional variants as needed.
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // Map error variants to appropriate status codes.
        let status = match self {
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            //::BadRequest(_) => StatusCode::BAD_REQUEST,
        };

        // You could also return a JSON body with more details if desired.
        let body = format!("{{\"error\": \"{}\"}}", self);
        (status, body).into_response()
    }
}
