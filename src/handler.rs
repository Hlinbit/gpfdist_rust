
use crate::http_parser::{GpfdistHeader, header_init};
use crate::session::Session;
use std::sync::Arc;
use std::vec::Vec;
use std::path::Path;
use hyper::{Body, Request, Response};
use crate::error::RequestError;
use crate::file_stream::read_file_lines;
use crate::{GLOBAL_CONFIG, GLOBAL_SESSION_HUB};
pub struct GpfdistRequest {
    pub http: GpfdistHeader,
    pub buf: Vec<u8>,
    pub file_name: String,
    pub tid: String,
}

impl GpfdistRequest {
    pub fn init(req: Request<Body>) -> Result<GpfdistRequest, RequestError> {
        let mut header = header_init();
        header.parse(&req)?;
        let file_name = String::from(Path::new(req.uri().path()).file_name().and_then(|f| f.to_str()).unwrap_or(req.uri().path()));
        let dir = GLOBAL_CONFIG.directory.as_ref().unwrap().as_path();
        let path_buf = dir.join(&file_name);
        let file_path: &Path = path_buf.as_path();

        let sn = header.sn.as_ref().map_or_else(|| "".to_string(), |s| s.clone());
        let cid = header.cid.as_ref().map_or_else(|| "".to_string(), |s| s.clone());
        let xid = header.xid.as_ref().map_or_else(|| "".to_string(), |s| s.clone());
        let gp_proto =  header.gp_proto.as_ref().map_or_else(|| "0".to_string(), |s| s.clone());
        let tid = format!("{}:{}:{}:{}", cid, xid, sn, gp_proto);
        return Ok(GpfdistRequest{
            http: header,
            buf: Vec::with_capacity(GLOBAL_CONFIG.max_data_row_length.unwrap_or(1024 * 64 as usize)),
            file_name: String::from(file_path.to_str().ok_or("Path that cannot be converted to String.").map_err(|err| RequestError::InternalErrorType(err.to_string()))?),
            tid: tid,
        })
    }
}

fn session_attach(request: &GpfdistRequest) {
    let session_t = Arc::new(Session{
        offset: 0u64,
        file_path: request.file_name.clone(),
        key: request.tid.clone(),
    });
    {
        let mut map = GLOBAL_SESSION_HUB.lock().unwrap();
        map.insert(session_t.key.clone(), session_t);
    }
}

pub fn get_hander (req: Request<Body>) -> Result<Response<Body>, RequestError> {
    let mut request = GpfdistRequest::init(req)?;
    
    let n_byte = GLOBAL_CONFIG.max_data_row_length.unwrap() as usize;
    let file_path = Path::new(request.file_name.as_str());
    session_attach(&request);
    let content_length = read_file_lines(n_byte, "\n", file_path, 0, &mut request.buf).
            map_err(|err| RequestError::InternalErrorType(err.to_string()))?;
    if content_length == 0 {
        return Err(RequestError::InternalErrorType(String::from("Max line length is not enough to contain a line.")));
    }
    let result = String::from_utf8(request.buf)
            .map_err(|e| RequestError::InternalErrorType(e.to_string()))?;

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