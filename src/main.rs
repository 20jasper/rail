mod commands;
mod error;

use error::Result;
use rail::{read_bytes_end, tail_file};

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

    if matches.get_flag("follow") {
        listen_for_modifications(&mut f, path)?;
    }

    Ok(())
}

use notify::{
    event::{AccessKind, AccessMode},
    Event, EventKind, RecursiveMode, Watcher,
};
use std::{
    fs::File,
    io::{stdout, Seek, Write},
    path::{Path, PathBuf},
    sync::mpsc,
};

fn listen_for_modifications(f: &mut File, path: &Path) -> notify::Result<()> {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    f.seek(std::io::SeekFrom::End(0))?;
    let mut len = f.metadata()?.len();

    for res in rx {
        match res {
            Ok(Event { kind, .. }) => match kind {
                EventKind::Modify(_) => {
                    let new_len = f.metadata()?.len();
                    if new_len <= len + 1 {
                        // todo, if file is less, print whole file since don't know what happened
                        // println!("same len or less");
                        continue;
                    }

                    let diff = new_len - len;
                    len = new_len;

                    let vec = read_bytes_end(f, diff as i64)?;
                    print!("{}", String::from_utf8(vec).expect("should be valid utf8"));
                    stdout().flush()?;
                }
                EventKind::Access(AccessKind::Close(AccessMode::Write)) => {
                    let new_len = f.metadata()?.len();
                    if new_len != len {
                        len = new_len;

                        let vec = read_bytes_end(f, new_len as i64)?;
                        print!("{}", String::from_utf8(vec).expect("should be valid utf8"));
                        stdout().flush()?;
                    }
                }
                _ => (),
            },
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }

    Ok(())
}
