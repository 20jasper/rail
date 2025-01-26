use core::error::Error;
use core::time::Duration;
use std::fs::File;
use std::io::Write;
use std::thread::sleep;

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::options().append(true).open("test.txt")?;

    for i in 0..10 {
        writeln!(f, "{i}")?;
        sleep(Duration::from_secs(1));
    }

    Ok(())
}
