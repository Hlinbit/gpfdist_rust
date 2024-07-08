use std::convert::TryInto;
use std::mem::size_of;
use std::net::Ipv4Addr;

struct Request;
struct Block {
    hdr: BlockHeader,
    top: usize,
    bot: usize,
}

struct BlockHeader {
    hbyte: Vec<u8>,
    htop: usize,
    hbot: usize,
}

struct FilenameAndOffset {
    fname: String,
    foff: u64,
    line_number: u64,
}

fn local_htonl(val: u32) -> u32 {
    val.to_be()
}

fn local_htonll(val: u64) -> u64 {
    val.to_be()
}

fn block_fill_header(r: &Request, b: &mut Block, fos: &FilenameAndOffset) {
    let h = &mut b.hdr;
    let mut p = &mut h.hbyte;

    h.hbot = 0;

    // FILENAME: 'F' + len + fname
    p.push(b'F');
    let fname_len = fos.fname.len() as u32;
    p.extend_from_slice(&local_htonl(fname_len).to_ne_bytes());
    p.extend_from_slice(fos.fname.as_bytes());

    // OFFSET: 'O' + len + foff
    p.push(b'O');
    p.extend_from_slice(&local_htonl(8).to_ne_bytes());
    p.extend_from_slice(&local_htonll(fos.foff).to_ne_bytes());

    // LINENUMBER: 'L' + len + linenumber
    p.push(b'L');
    p.extend_from_slice(&local_htonl(8).to_ne_bytes());
    p.extend_from_slice(&local_htonll(fos.line_number).to_ne_bytes());

    // DATA: 'D' + len
    p.push(b'D');
    let data_len = b.top - b.bot;
    p.extend_from_slice(&local_htonl(data_len as u32).to_ne_bytes());

    h.htop = p.len();
    if h.htop > h.hbyte.capacity() {
        panic!(
            "assert failed, h->htop = {}, max = {}",
            h.htop,
            h.hbyte.capacity()
        );
    }
}

use std::str::from_utf8;
use std::collections::HashMap;

#[derive(Debug)]
struct GnetRequest {
    argc: usize,
    argv: Vec<String>,
    headers: HashMap<String, String>,
}

fn gnet_parse_request(buf: &[u8]) -> Option<GnetRequest> {
    let buf_str = from_utf8(buf).ok()?;
    let mut lines = buf_str.split("\n");
    let mut request = GnetRequest {
        argc: 0,
        argv: Vec::new(),
        headers: HashMap::new(),
    };

    // Find the empty line that separates headers from the body
    let mut empty = true;
    let mut completed = false;
    let mut len = 0;
    for line in buf_str.lines() {
        len += line.len() + 1; // include the newline character
        if line.trim().is_empty() {
            if !empty {
                empty = true;
                continue;
            }
            completed = true;
            break;
        }
        empty = false;
    }

    if !completed {
        return None; // Not a complete request
    }

    // First line (request line)
    if let Some(line) = lines.next() {
        let line_trimmed = line.trim();
        if !line_trimmed.is_empty() {
            request.argv = line_trimmed.split_whitespace().map(String::from).collect();
            request.argc = request.argv.len();
        }
    }

    // Header lines
    let mut current_header_value = String::new();
    while let Some(mut line) = lines.next() {
        line = line.trim_end();
        if line.is_empty() {
            break;
        }

        if line.starts_with(' ') || line.starts_with('\t') {
            // Continuation of the previous header value
            current_header_value.push(' ');
            current_header_value.push_str(line.trim_start());
        } else {
            // New header line
            if !current_header_value.is_empty() {
                if let Some((key, value)) = current_header_value.split_once(':') {
                    request.headers.insert(key.trim().to_string(), value.trim().to_string());
                }
                current_header_value.clear();
            }

            current_header_value.push_str(line);
        }
    }

    // Insert the last header if any
    if !current_header_value.is_empty() {
        if let Some((key, value)) = current_header_value.split_once(':') {
            request.headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Some(request)
}