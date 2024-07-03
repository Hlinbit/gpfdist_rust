mod header_parser;
mod error;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use header_parser::ParsedHeaders;
use error::{error_response, RequestError};


async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let result: Result<Response<Body>, RequestError> = process_request(req).await;
    match result {
        Ok(response) => Ok(response),
        Err(err) => Ok(error_response(&err)),
    }
}

async fn process_request(req: Request<Body>) -> Result<Response<Body>, RequestError> {
    let mut parsed_headers = ParsedHeaders::init();
    parsed_headers.parse(req)?;
    let json_response: String = serde_json::to_string(&parsed_headers)
    .map_err(RequestError::JsonSerializationError)?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(json_response))
        .unwrap())
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8091));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}