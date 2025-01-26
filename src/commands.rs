use std::path::PathBuf;

use clap::{arg, ArgGroup, Command};

pub fn cli() -> Command {
    Command::new("rail")
        .about("Print the end of a file")
        .arg_required_else_help(true)
        .arg(arg!(<PATH> ... "File to read from").value_parser(clap::value_parser!(PathBuf)))
        .arg(arg!(-c --bytes <COUNT>).value_parser(clap::value_parser!(i64)))
        .arg(
            arg!(-n --lines <COUNT>)
                .default_value("10")
                .value_parser(clap::value_parser!(usize)),
        )
        .group(ArgGroup::new("measurement").args(["bytes", "lines"]))
        .arg(arg!(-f --follow "follow file descriptor"))
}
