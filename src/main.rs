mod commands;
mod error;

use rail::{read_bytes_end, tail_file};

use error::Result;
use notify::{
    event::{AccessKind, AccessMode},
    Event, EventKind, RecursiveMode, Watcher,
};
use std::{
    fs::File,
    io::{stdout, Write},
    path::{Path, PathBuf},
    sync::mpsc,
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

    if matches.get_flag("follow") {
        listen_for_modifications(&mut f, path)?;
    }

    Ok(())
}

fn listen_for_modifications(f: &mut File, path: &Path) -> notify::Result<()> {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    let mut len = f.metadata()?.len();

    for res in rx {
        match res {
            Ok(Event { kind, .. }) => match kind {
                EventKind::Modify(_) => {
                    let new_len = f.metadata()?.len();
                    // account for new line
                    if new_len <= len + 1 {
                        continue;
                    }

                    let diff = new_len - len;
                    len = new_len;

                    let vec = read_bytes_end(f, diff as i64)?;
                    print!("{}", String::from_utf8(vec).expect("should be valid utf8"));
                    stdout().flush()?;
                }
                EventKind::Access(AccessKind::Close(AccessMode::Write)) => {
                    println!("rail: file truncated");

                    let new_len = f.metadata()?.len();
                    len = new_len;

                    let vec = read_bytes_end(f, new_len as i64)?;
                    print!("{}", String::from_utf8(vec).expect("should be valid utf8"));
                    stdout().flush()?;
                }
                _ => (),
            },
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }

    Ok(())
}
