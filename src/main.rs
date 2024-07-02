use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use serde::Serialize;

#[derive(Serialize)]
struct ParsedHeaders {
    xid: Option<String>,
    cid: Option<String>,
    sn: Option<String>,
    csvopt: Option<String>,
    gp_proto: Option<String>,
    is_final: bool,
    totalsegs: Option<i32>,
    segid: Option<i32>,
    zstd: Option<i32>,
    line_delim_str: Option<String>,
    line_delim_length: Option<i32>,
    trans_name: Option<String>,
    seq: Option<i64>,
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut parsed_headers = ParsedHeaders {
        xid: None,
        cid: None,
        sn: None,
        csvopt: None,
        gp_proto: None,
        is_final: false,
        totalsegs: None,
        segid: None,
        zstd: None,
        line_delim_str: None,
        line_delim_length: None,
        trans_name: None,
        seq: None,
    };

    for (name, value) in req.headers() {
        if let Ok(value_str) = value.to_str() {
            match name.as_str() {
                "x-gp-xid" => parsed_headers.xid = Some(value_str.to_string()),
                "x-gp-cid" => parsed_headers.cid = Some(value_str.to_string()),
                "x-gp-sn" => parsed_headers.sn = Some(value_str.to_string()),
                "x-gp-csvopt" => parsed_headers.csvopt = Some(value_str.to_string()),
                "x-gp-proto" => parsed_headers.gp_proto = Some(value_str.to_string()),
                "x-gp-done" => parsed_headers.is_final = true,
                "x-gp-segment-count" => parsed_headers.totalsegs = value_str.parse().ok(),
                "x-gp-segment-id" => parsed_headers.segid = value_str.parse().ok(),
                "x-gp-zstd" => parsed_headers.zstd = value_str.parse().ok(),
                "x-gp-line-delim-str" => parsed_headers.line_delim_str = Some(value_str.to_string()),
                "x-gp-line-delim-length" => parsed_headers.line_delim_length = value_str.parse().ok(),
                "x-gp-transform" => parsed_headers.trans_name = Some(value_str.to_string()),
                "x-gp-seq" => {
                    if let Ok(seq) = value_str.parse::<i64>() {
                        if seq > 0 {
                            parsed_headers.seq = Some(seq);
                        } else {
                            return Ok(Response::builder()
                                .status(400)
                                .body(Body::from("Invalid sequence number"))
                                .unwrap());
                        }
                    }
                }
                _ => {println!("{}, {}", name.as_str(), value_str);}
            }
        }
    }

    let json_response = serde_json::to_string(&parsed_headers).unwrap();

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(json_response))
        .unwrap())
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8090));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // 运行服务器
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}