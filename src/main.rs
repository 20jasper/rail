mod commands;
mod error;

use error::Result;
use rail::{read_bytes_end, tail_file};

fn main() -> Result<()> {
    let matches = commands::cli().get_matches();

    let s = tail_file(&matches)?;
    print!("{s}");

    if matches.get_flag("follow") {
        listen_for_modifications(matches.get_one::<PathBuf>("PATH").unwrap())?;
    }

    Ok(())
}

use notify::{
    event::{AccessKind, AccessMode},
    Event, EventKind, RecursiveMode, Watcher,
};
use std::{
    fs::File,
    io::{stdout, Read, Seek, Write},
    path::{Path, PathBuf},
    sync::mpsc,
};

fn listen_for_modifications(path: &Path) -> notify::Result<()> {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    let mut f = File::open(path)?;
    f.seek(std::io::SeekFrom::End(0))?;
    let mut len = f.metadata()?.len();

    // Block forever, printing out events as they come in
    for res in rx {
        match res {
            Ok(Event {
                kind: EventKind::Modify(ref kind),
                attrs,
                ..
            }) => {
                // println!("kind: {:?}. Attrs {attrs:?}", kind);
                let new_len = f.metadata()?.len();

                println!("len, {len} new_len {new_len}");
                if new_len <= len + 1 {
                    // todo, if file is less, print whole file since don't know what happened
                    // println!("same len or less");
                    continue;
                }

                let diff = dbg!(new_len - len + 1);
                len = new_len;

                let vec = read_bytes_end(&mut f, diff as i64)?;
                dbg!(&vec);
                if vec.iter().any(|b| b == &'\n') {}

                print!("{}", String::from_utf8(vec).expect("shold be valid utf8"));
                stdout().flush()?;
            }
            Ok(Event {
                kind: EventKind::Access(AccessKind::Close(AccessMode::Write)),
                attrs,
                ..
            }) => {
                let new_len = f.metadata()?.len();
                if new_len != len {
                    len = new_len;
                    // dbg!("File written and closed");

                    let vec = read_bytes_end(&mut f, new_len as i64)?;
                    print!("{}", String::from_utf8(vec).expect("shold be valid utf8"));
                    stdout().flush()?;
                } else {
                    println!("no need to write file, unmodified");
                }
            }
            // Ok(Event { kind, attrs, .. }) => println!("{kind:?}, {attrs:?}"),
            Err(e) => println!("watch error: {:?}", e),
            _ => (),
        }
    }

    Ok(())
}
