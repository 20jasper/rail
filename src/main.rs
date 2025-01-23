use core::error::Error;
use std::{
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = commands::cli().get_matches();

    let path = matches.get_one::<PathBuf>("PATH").unwrap();
    let bytes = *matches.get_one::<i64>("bytes").unwrap();

    let mut f = File::open(path)?;
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

    use clap::{arg, Command};

    pub fn cli() -> Command {
        Command::new("rail")
            .about("Print the end of a file")
            .arg_required_else_help(true)
            .arg(arg!(<PATH> ... "File to pull from").value_parser(clap::value_parser!(PathBuf)))
            .arg(
                arg!(-c --bytes <COUNT>)
                    .default_value("10")
                    .value_parser(clap::value_parser!(i64)),
            )
    }
}
