use rail::commands;
use rail::tail_file;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[test]
fn get_bytes() -> Result<()> {
    let matches =
        commands::cli().get_matches_from(["rail", "--bytes", "10", "tests/assets/test.txt"]);

    assert_eq!(tail_file(matches)?, " laborum.\n");

    Ok(())
}
