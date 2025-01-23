use core::error::Error;
use std::{
    fs::File,
    io::{Read, Seek},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    args.next();
    let file = args.next().unwrap();
    args.next();
    let bytes = args
        .next()
        .and_then(|x| x.parse::<i64>().ok())
        .unwrap_or(10);

    let mut f = File::open(file)?;
    let max_len = f.metadata()?.len();
    let len = (bytes).clamp(0, max_len as i64);

    let mut buf = vec![0; len.try_into().unwrap()];
    f.seek(std::io::SeekFrom::End(-len))?;
    f.read_exact(&mut buf)?;
    print!("{}", std::str::from_utf8(&buf)?);

    Ok(())
}
