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
        dbg!(bytes);
        read_bytes_end(&mut f, bytes)?
    } else if let Some(&lines) = matches.get_one::<i64>("lines") {
        dbg!(lines);
        read_lines_end(&mut f, lines)?
    } else {
        unreachable!("must have lines or bytes passed")
    };

    Ok(s)
}

fn read_lines_end(f: &mut File, lines: i64) -> Result<String> {
    let mut buf = String::default();
    f.read_to_string(&mut buf)?;
    let count = buf.lines().count();
    Ok(buf
        .lines()
        .skip(count - lines as usize)
        .collect::<Vec<_>>()
        .join("\n")
        + "\n")
}

fn read_bytes_end(f: &mut File, bytes: i64) -> Result<String> {
    let max_len = f.metadata()?.len();
    let len = (bytes).clamp(0, max_len as i64);
    let mut buf = vec![0; len as usize];
    f.seek(std::io::SeekFrom::End(-len))?;
    f.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}
