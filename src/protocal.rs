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