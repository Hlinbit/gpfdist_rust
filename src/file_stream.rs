use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::{Arc, Mutex};
use crate::session::Session;
use crate::protocal::block_fill_header;

pub fn read_file_lines(
    n_byte: usize,
    line_delimiter: &str,
    file_path: &Path,
    s:  &mut Arc<Mutex<Session>>,
    buf: &mut Vec<u8>,
) -> Result<usize, std::io::Error> {
    let offset = s.lock().unwrap().offset;
    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::Start(offset))?;

    let mut find_end = false;
    let line_delimiter_bytes: &[u8] = line_delimiter.as_bytes();
    let line_delimiter_len = line_delimiter_bytes.len();

    let mut chunk = vec![0u8; n_byte];
    let bytes_read = file.read(&mut chunk)?;
    let mut content_length = bytes_read;
    let mut total_length = 0usize;

    if bytes_read == n_byte {
        let mut i =  bytes_read - line_delimiter_len;
        while i > 0 && !find_end {
            if chunk[i..].starts_with(line_delimiter_bytes) {
                find_end = true;
                content_length = i + line_delimiter_len;
                break;
            }
            i -= 1;
        }
    } else {
        find_end = true
    }

    if find_end {
        let start = 0;
        let gpfdist_header = fill_gpfdist_header(offset, file_path.to_str().unwrap(), 0, content_length as u64);
        let head_length = gpfdist_header.len();
        total_length = content_length + head_length;
        s.lock().unwrap().offset += content_length as u64;
        buf[start..head_length].copy_from_slice(&gpfdist_header[0..head_length]);
        buf[head_length..total_length].copy_from_slice(&chunk[0..content_length]);
        buf.truncate(total_length);
    }
        

    Ok(total_length)
}

fn fill_gpfdist_header(offset: u64, f_name: &str, l_num: u64, data_len: u64) -> Vec<u8> {
    block_fill_header(f_name, offset, l_num, data_len)
}