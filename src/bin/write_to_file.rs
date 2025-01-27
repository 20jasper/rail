use core::error::Error;
use core::time::Duration;
use std::env;
use std::fs::File;
use std::io::Write;
use std::thread::sleep;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let lines = args.next().unwrap_or("true".to_owned()).parse::<bool>()?;
    let path = args.next().unwrap_or("test.txt".to_owned());

    let mut f = File::options().append(true).open(path)?;

    if lines {
        for i in 0..10 {
            writeln!(f, "{i}")?;
            sleep(Duration::from_millis(500));
            println!("wriet lines");
        }
        println!("done with lines");
    } else {
        for _ in 0..10 {
            write!(&mut f, "hi")?;
            f.flush()?;
            sleep(Duration::from_millis(500));
            println!("wriet char");
        }
    }

    println!("done");

    Ok(())
}
