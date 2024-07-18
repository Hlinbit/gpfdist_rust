use thiserror::Error;
use hyper::{Body, Response};
#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Invalid header value for {0}")]
    InvalidHeaderValue(String),
    #[error("Failed to serialize JSON response")]
    JsonSerializationError(#[from] serde_json::Error),
    #[error("Invalid sequence number")]
    InvalidSequenceNumber,
    #[error("Invalid request type {0}")]
    InvalidRequestType(String),
    #[error("Internal error: {0}")]
    InternalErrorType(String),
}

pub fn error_response(err: &RequestError) -> Response<Body>{
    match err {
        RequestError::InvalidHeaderValue(_) => Response::builder()
            .status(400)
            .body(Body::from(err.to_string()))
            .unwrap(),
        RequestError::InvalidSequenceNumber => Response::builder()
            .status(400)
            .body(Body::from("Invalid sequence number"))
            .unwrap(),
        RequestError::JsonSerializationError(_) => Response::builder()
            .status(500)
            .body(Body::from("Internal server error: Failed to serialize JSON response"))
            .unwrap(),
        RequestError::InvalidRequestType(_) => Response::builder()
            .status(404)
            .body(Body::from(err.to_string()))
            .unwrap(),
        RequestError::InternalErrorType(_) => Response::builder()
            .status(404)
            .body(Body::from(err.to_string()))
            .unwrap(),
    }
}