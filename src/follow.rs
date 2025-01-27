use notify::{
    event::{AccessKind, AccessMode},
    Event, EventKind, RecursiveMode, Watcher,
};
use std::{
    fs::File,
    io::{Seek, Write},
    path::Path,
    sync::mpsc,
};

use crate::read_bytes_end;

pub fn listen_for_modifications(
    f: &mut File,
    path: &Path,
    w: &mut impl Write,
) -> notify::Result<()> {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    f.seek(std::io::SeekFrom::End(0))?;
    let mut len = f.metadata()?.len();

    for res in rx {
        // TODO, pull this out for testing purposes
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
                    write!(
                        w,
                        "{}",
                        String::from_utf8(vec).expect("should be valid utf8")
                    )?;
                    w.flush()?;
                }
                EventKind::Access(AccessKind::Close(AccessMode::Write)) => {
                    println!("rail: file truncated");

                    let new_len = f.metadata()?.len();
                    len = new_len;

                    let vec = read_bytes_end(f, new_len as i64)?;
                    write!(
                        w,
                        "{}",
                        String::from_utf8(vec).expect("should be valid utf8")
                    )?;
                    w.flush()?;
                }
                _ => (),
            },
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }

    Ok(())
}
