
use crate::http_parser::{GpfdistHeader, header_init};
use std::vec::Vec;
use std::path::Path;
use hyper::{Body, Request, Response};
use crate::error::RequestError;
use crate::file_stream::read_file_lines;
use crate::GLOBAL_CONFIG;
pub struct GpfdistRequest {
    pub http: GpfdistHeader,
    pub buf: Vec<u8>,
    pub file_name: String,
}

impl GpfdistRequest {
    pub fn init(req: Request<Body>) -> Result<GpfdistRequest, RequestError> {
        let mut header = header_init();
        header.parse(&req)?;
        return Ok(GpfdistRequest{
            http: header,
            buf: vec![0u8; GLOBAL_CONFIG.max_data_row_length.unwrap() as usize],
            file_name: String::from(Path::new(req.uri().path()).file_name().and_then(|f| f.to_str()).unwrap_or(req.uri().path())),
        })
    }
}

pub fn get_hander (req: Request<Body>) -> Result<Response<Body>, RequestError> {
    let mut request = GpfdistRequest::init(req)?;
    let n_byte = GLOBAL_CONFIG.max_data_row_length.unwrap() as usize;
    let fname = request.file_name;
    let dir = GLOBAL_CONFIG.directory.as_ref().unwrap().as_path();
    let path_buf = dir.join(&fname);
    let file_path = path_buf.as_path();
    println!("{:?}", file_path);

    read_file_lines(n_byte, "\n", file_path, 0, &mut request.buf).
            map_err(|err| RequestError::InternalErrorType(err.to_string()))?;
    let result = String::from_utf8(request.buf)
            .map_err(|e| RequestError::InternalErrorType(e.to_string()))?;
        
    // let json_response: String = serde_json::to_string(&result)
    // .map_err(RequestError::JsonSerializationError)?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(result))
        .unwrap())
}

pub fn post_hander (req: Request<Body>) -> Result<Response<Body>, RequestError> {
    let request = GpfdistRequest::init(req)?;
    let json_response: String = serde_json::to_string(&request.http)
    .map_err(RequestError::JsonSerializationError)?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(json_response))
        .unwrap())
}