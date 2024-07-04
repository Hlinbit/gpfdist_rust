use serde::Serialize;
use hyper::{Body, Request};
use crate::error::RequestError;

#[derive(Serialize)]
pub struct ParsedHeaders {
    pub xid: Option<String>,
    pub cid: Option<String>,
    pub sn: Option<String>,
    pub csvopt: Option<String>,
    pub gp_proto: Option<String>,
    pub is_final: bool,
    pub totalsegs: Option<i32>,
    pub segid: Option<i32>,
    pub zstd: Option<i32>,
    pub line_delim_str: Option<String>,
    pub line_delim_length: Option<i32>,
    pub trans_name: Option<String>,
    pub seq: Option<i64>,
}

impl ParsedHeaders {
    pub fn init() -> ParsedHeaders {
        return ParsedHeaders {
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
    }

    pub fn parse(&mut self ,req: Request<Body>) -> Result<bool, RequestError> {
        for (name, value) in req.headers() {
            let value_str = value.to_str().map_err(|_| RequestError::InvalidHeaderValue(name.to_string()))?;
            match name.as_str() {
                "x-gp-xid" => self.xid = Some(value_str.to_string()),
                "x-gp-cid" => self.cid = Some(value_str.to_string()),
                "x-gp-sn" => self.sn = Some(value_str.to_string()),
                "x-gp-csvopt" => self.csvopt = Some(value_str.to_string()),
                "x-gp-proto" => self.gp_proto = Some(value_str.to_string()),
                "x-gp-done" => self.is_final = true,
                "x-gp-segment-count" => self.totalsegs = Some(value_str.parse().map_err(|_| RequestError::InvalidHeaderValue(name.to_string()))?),
                "x-gp-segment-id" => self.segid = Some(value_str.parse().map_err(|_| RequestError::InvalidHeaderValue(name.to_string()))?),
                "x-gp-zstd" => self.zstd = Some(value_str.parse().map_err(|_| RequestError::InvalidHeaderValue(name.to_string()))?),
                "x-gp-line-delim-str" => self.line_delim_str = Some(value_str.to_string()),
                "x-gp-line-delim-length" => self.line_delim_length = Some(value_str.parse().map_err(|_| RequestError::InvalidHeaderValue(name.to_string()))?),
                "x-gp-transform" => self.trans_name = Some(value_str.to_string()),
                "x-gp-seq" => {
                    if let Ok(seq) = value_str.parse::<i64>() {
                        if seq > 0 {
                            self.seq = Some(seq);
                        } else {
                            return Err(RequestError::InvalidSequenceNumber);
                        }
                    } else {
                        return Err(RequestError::InvalidHeaderValue(name.to_string()));
                    }
                }
                _ => {println!("{}, {}", name.as_str(), value_str);}
            }
        }
        return Ok(true);
    }
}