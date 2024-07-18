use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub fn read_file_lines(
    n_byte: usize,
    line_delimiter: &str,
    file_path: &Path,
    offset: u64,
    buf: &mut Vec<u8>,
) -> Result<usize, std::io::Error> {
    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::Start(offset))?;

    let mut find_end = false;
    let line_delimiter_bytes: &[u8] = line_delimiter.as_bytes();
    let line_delimiter_len = line_delimiter_bytes.len();

    let mut chunk = vec![0u8; n_byte];
    let bytes_read = file.read(&mut chunk)?;
    let mut content_length = bytes_read;

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
        let end = content_length;
        let start = 0;
        buf[start..end].copy_from_slice(&chunk[0..content_length]);
    }
        

    Ok(content_length)
}