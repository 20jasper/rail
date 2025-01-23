use core::error::Error;
use std::{
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = commands::cli().get_matches();

    let path = matches
        .get_one::<PathBuf>("PATH")
        .expect("path is required");
    let mut f = File::open(path)?;

    if let Some(&bytes) = matches.get_one::<i64>("bytes") {
        dbg!(bytes);
        read_bytes_end(&mut f, bytes)?;
    } else if let Some(&lines) = matches.get_one::<i64>("lines") {
        dbg!(lines);
        todo!("make lines function")
    } else {
        unreachable!("must have lines or bytes passed")
    };

    Ok(())
}

fn read_bytes_end(f: &mut File, bytes: i64) -> Result<(), Box<dyn Error>> {
    let max_len = f.metadata()?.len();
    let len = (bytes).clamp(0, max_len as i64);
    let mut buf = vec![0; len.try_into().unwrap()];
    f.seek(std::io::SeekFrom::End(-len))?;
    f.read_exact(&mut buf)?;
    print!("{}", std::str::from_utf8(&buf)?);
    Ok(())
}

mod commands {
    use std::path::PathBuf;

    use clap::{arg, ArgGroup, Command};

    pub fn cli() -> Command {
        Command::new("rail")
            .about("Print the end of a file")
            .arg_required_else_help(true)
            .arg(arg!(<PATH> ... "File to pull from").value_parser(clap::value_parser!(PathBuf)))
            .arg(arg!(-c --bytes <COUNT>).value_parser(clap::value_parser!(i64)))
            .arg(
                arg!(-n --lines <COUNT>)
                    .default_value("10")
                    .value_parser(clap::value_parser!(i64)),
            )
            .group(ArgGroup::new("measurement").args(["bytes", "lines"]))
    }
}
