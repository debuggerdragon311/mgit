/// program
/// |---commands
/// |---subcommands(optional)
///      |--- options/flags
///      |--- args

use std::env;

mod init;

fn main() {
    let args: Vec<String> = env::args().collect();

    // global flags
    if args.len() < 2 {
        eprintln!("Usage: mgit <COMMAND> [ARGUMENTS]");
        return;
    }

    match args[1].as_str() {
        "init" => init::run(),
        "-h" | "--help" => println!("{}", r#"
mgit
  A version control system for tracking file changes.

USAGE:
    mgit [-v | --version] [-h | --help] <COMMANDS> <SUBCOMMANDS>

COMMANDS:
    init      Create an empty mgit repository or reinitialize an existing one
    add       Add file contents to the staging index
    commit    Record changes to the repository history
    status    Show the working tree status

Run 'mgit <COMMANDS> --help' for more information on a specific command.
        "#),

        "-v" | "--version" => println!("mgit: v0.0.1"),

        _ => eprintln!("Error: Unknown command '{}'", args[1]),
    }
}
