use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use rail::commands;
use rail::follow;
use rail::tail_file;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn tail_file_wrapped(matches: &clap::ArgMatches) -> Result<String> {
    let path = matches.get_one::<PathBuf>("PATH").unwrap();
    let mut f = File::open(path).unwrap();

    tail_file(matches, &mut f)
}

#[test]
fn get_bytes() -> Result<()> {
    let matches =
        commands::cli().get_matches_from(["rail", "--bytes", "10", "tests/assets/test.txt"]);

    assert_eq!(tail_file_wrapped(&matches)?, " laborum.\n");

    Ok(())
}

#[test]
fn defaults_to_ten() -> Result<()> {
    let matches = commands::cli().get_matches_from(["rail", "tests/assets/test.txt"]);

    assert_eq!(tail_file_wrapped(&matches)?, "3\n4\n5\n6\n7\n8\n9\n10\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \nDuis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n");

    Ok(())
}

#[test]
fn should_return_whole_file() -> Result<()> {
    let matches = commands::cli().get_matches_from(["rail", "-n", "1000", "tests/assets/test.txt"]);

    assert_eq!(tail_file_wrapped(&matches)?, "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \nDuis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n");

    Ok(())
}

#[test]
fn file_bigger_than_buffer() -> Result<()> {
    let matches =
        commands::cli().get_matches_from(["rail", "-n", "100000", "tests/assets/11kb-test.txt"]);

    assert_eq!(tail_file_wrapped(&matches)?, "Rust is a must\n".repeat(759));

    Ok(())
}

#[test]
fn follow_mode() -> Result<()> {
    let path = Path::new("tests/assets/writeable/to_write.txt");
    let mut f = File::open(path).unwrap();
    let mut w = vec![0; 1000];

    follow::listen_for_modifications(&mut f, path, &mut w)?;

    writeln!(f, "hello")?;
    writeln!(f, "hello")?;
    writeln!(f, "hello")?;

    write!(f, "1")?;
    write!(f, "2")?;
    write!(f, "3")?;

    assert_eq!(w, []);

    Ok(())
}
