mod commands;
mod error;

use error::Result;
use rail::tail_file;

fn main() -> Result<()> {
    let matches = commands::cli().get_matches();

    let s = tail_file(matches)?;
    print!("{s}");

    Ok(())
}
