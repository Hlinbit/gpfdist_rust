
use crate::header_parser::ParsedHeaders;
use std::vec::Vec;
use hyper::{Body, Request, Response};
use crate::error::{error_response, RequestError};
pub struct get_request {
    pub http: ParsedHeaders,
    pub buf: Vec<u8>,
}

pub struct post_request {
    pub http: ParsedHeaders,
    pub buf: Vec<u8>,
}

pub fn get_hander (req: Request<Body>) -> Result<Response<Body>, RequestError> {
    let mut parsed_headers = ParsedHeaders::init();
    parsed_headers.parse(req)?;
    let json_response: String = serde_json::to_string(&parsed_headers)
    .map_err(RequestError::JsonSerializationError)?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(json_response))
        .unwrap())
}

pub fn post_hander (req: Request<Body>) -> Result<Response<Body>, RequestError> {
    let mut parsed_headers = ParsedHeaders::init();
    parsed_headers.parse(req)?;
    let json_response: String = serde_json::to_string(&parsed_headers)
    .map_err(RequestError::JsonSerializationError)?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(json_response))
        .unwrap())
}