mod http_parser;
mod error;
mod flag_parser;
mod file_stream;
mod handler;
mod protocal;
mod session;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Method};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::process::exit;
use handler::{get_hander, post_hander};
use error::{error_response, RequestError};
use flag_parser::{parse_flags, Opt};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use session::Session;
use std::collections::HashMap;

static GLOBAL_CONFIG: Lazy<Arc<Opt>> = Lazy::new(|| {
    match parse_flags(){
        Ok(opt) => Arc::new(opt),
        Err(err) => {
            eprintln!("Failed to parse flags: {}", err);
            exit(1);
        }
    }
});

static GLOBAL_SESSION_HUB: Lazy<Mutex<HashMap<String, Arc<Session>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});


async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let result: Result<Response<Body>, RequestError> = process_request(req).await;
    match result {
        Ok(response) => Ok(response),
        Err(err) => Ok(error_response(&err)),
    }
}

async fn process_request(req: Request<Body>) -> Result<Response<Body>, RequestError> {
    let method = req.method().clone();
    match method {
        Method::GET => {get_hander(req)},
        Method::POST => {post_hander(req)},
        _ => {Err(RequestError::InvalidRequestType(method.to_string()))},
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], GLOBAL_CONFIG.port.unwrap()));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}