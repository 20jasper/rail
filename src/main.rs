mod commands;
mod error;

use rail::{follow, tail_file};

use error::Result;
use std::{
    fs::File,
    io::{stdout, Write},
    path::PathBuf,
};

fn main() -> Result<()> {
    let matches = commands::cli().get_matches();

    let path = matches
        .get_one::<PathBuf>("PATH")
        .expect("path is required");
    let mut f = {
        if !path.exists() {
            return Err(format!("path {path:?} does not exist").into());
        }
        if !path.is_file() {
            return Err(format!("path {path:?} is not a file").into());
        }
        File::open(path).map_err(|_| format!("Error opening {path:?}"))?
    };

    let s = tail_file(&matches, &mut f)?;
    print!("{s}");
    stdout().flush()?;

    let mut stdout = std::io::stdout();
    if matches.get_flag("follow") {
        follow::listen_for_modifications(&mut f, path, &mut stdout)?;
    }

    Ok(())
}
