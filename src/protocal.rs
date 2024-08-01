use std::convert::TryInto;
use std::mem::size_of;
use std::net::Ipv4Addr;


fn local_htonl(val: u32) -> u32 {
    val.to_be()
}

fn local_htonll(val: u64) -> u64 {
    val.to_be()
}

pub fn block_fill_header(f_name: &str, offset: u64, line_num: u64, data_len: u64) -> Vec<u8> {
    let mut p: Vec<u8> = Vec::with_capacity(1024);
    // FILENAME: 'F' + len + fname
    p.push(b'F');
    let fname_len = f_name.len() as u32;
    p.extend_from_slice(&local_htonl(fname_len).to_ne_bytes());
    p.extend_from_slice(f_name.as_bytes());

    // OFFSET: 'O' + len + foff
    p.push(b'O');
    p.extend_from_slice(&local_htonl(8).to_ne_bytes());
    p.extend_from_slice(&local_htonll(offset).to_ne_bytes());

    // LINENUMBER: 'L' + len + linenumber
    p.push(b'L');
    p.extend_from_slice(&local_htonl(8).to_ne_bytes());
    p.extend_from_slice(&local_htonll(line_num).to_ne_bytes());

    // DATA: 'D' + len
    p.push(b'D');
    p.extend_from_slice(&local_htonl(data_len as u32).to_ne_bytes());
    return p;
}