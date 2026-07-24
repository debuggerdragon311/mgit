/**************************************************************************/
/*                                main.rs                                 */
/**************************************************************************/
/*                         This file is part of:                          */
/*                                  mgit                                  */
/**************************************************************************/
/*                  Copyright (C) 2026  Soumyajit Bala                    */
/*                                                                        */
/* This program is free software: you can redistribute it and/or modify   */
/* it under the terms of the GNU Affero General Public License as         */
/* published by the Free Software Foundation, either version 3 of the     */
/* License, or (at your option) any later version.                        */
/*                                                                        */
/* This program is distributed in the hope that it will be useful,        */
/* but WITHOUT ANY WARRANTY; without even the implied warranty of         */
/* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          */
/* GNU Affero General Public License for more details.                    */
/*                                                                        */
/* You should have received a copy of the GNU Affero General Public       */
/* License along with this program.  If not, see                          */
/* <https://www.gnu.org/licenses/>.                                       */
/**************************************************************************/

/// program
/// |---commands
/// |---subcommands(optional)
///      |--- options/flags
///      |--- args

use std::env;

mod init;
mod add;
mod index;
mod status;

fn main() {
    let args: Vec<String> = env::args().collect();

    // global flags
    if args.len() < 2 {
        eprintln!("Usage: mgit <COMMAND> [ARGUMENTS]");
        return;
    }

    match args[1].as_str() {
        "init" => init::run(),
        "add" => {
            if args.len() < 3 {
                eprintln!("Error: 'mgit add' requires a file path.");
                eprintln!("Usage: mgit add <file>");
                return;
            }
            add::run(args[2].clone());
        },
        "status" => status::run(),
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
