pub mod commands;
pub mod error;

use std::{
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
};

use error::Result;

pub fn tail_file(matches: clap::ArgMatches) -> Result<String> {
    let path = matches
        .get_one::<PathBuf>("PATH")
        .expect("path is required");
    let mut f = File::open(path)?;

    let s = if let Some(&bytes) = matches.get_one::<i64>("bytes") {
        read_bytes_end(&mut f, bytes)?
    } else if let Some(&lines) = matches.get_one::<usize>("lines") {
        read_lines_end(&mut f, lines)?
    } else {
        unreachable!("must have lines or bytes passed")
    };

    Ok(s)
}

/// C std lib `BUFSIZE` says this is good so sounds good to me
const BUF_SIZE: usize = 8096;

fn read_lines_end(f: &mut File, mut lines: usize) -> Result<String> {
    let max_len = f.metadata()?.len() as usize;
    let len = (BUF_SIZE).clamp(0, max_len);
    let mut buf = vec![0; len];
    f.seek(std::io::SeekFrom::End(0))?;

    let mut bytes: usize = 0;
    for _ in 0..=(max_len / len) {
        let to_seek = -((len).min(max_len - bytes) as i64);
        f.seek_relative(to_seek)?;
        f.read_exact(&mut buf)?;
        f.seek_relative(to_seek)?;

        let (b, l) = nth_line(&buf, lines);
        bytes += b;
        lines -= l;
        if lines == 0 {
            break;
        }
    }

    let s = read_bytes_end(f, bytes as i64)?;

    Ok(s)
}

/// return byte offset and lines left until the nth line
fn nth_line(buf: &[u8], n: usize) -> (usize, usize) {
    let newline_indexes = buf
        .iter()
        .rev()
        .enumerate()
        .filter_map(|(i, &b)| (b == b'\n').then_some(i));

    if let Some(offset) = newline_indexes.clone().nth(n) {
        (offset, n)
    } else {
        (buf.len(), newline_indexes.clone().count())
    }
}

fn read_bytes_end(f: &mut File, bytes: i64) -> Result<String> {
    let max_len = f.metadata()?.len();
    let len = (bytes).clamp(0, max_len as i64);
    let mut buf = vec![0; len as usize];
    f.seek(std::io::SeekFrom::End(-len))?;
    f.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}
