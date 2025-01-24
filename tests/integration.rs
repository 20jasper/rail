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

#[test]
fn defaults_to_ten() -> Result<()> {
    let matches = commands::cli().get_matches_from(["rail", "tests/assets/test.txt"]);

    assert_eq!(tail_file(matches)?, "3\n4\n5\n6\n7\n8\n9\n10\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \nDuis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n");

    Ok(())
}
