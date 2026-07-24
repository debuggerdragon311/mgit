# mgit

An experimental, simplified version control system designed to track file changes and mimic core Git workflows.

**Current Version:** v0.0.1 (Pre-alpha)

---

## Features & Status

*   **Repository Initialization** (`mgit init`): Creates the internal database structure to track your files.
*   **Staging** (`mgit add`): Adds file contents to the staging index.
*   **Status Tracking** (`mgit status`): Shows differences between the working directory and the staging area.
*   **Recording Changes** (`mgit commit`): *Currently under active development (Work in Progress).*

---

### Command Line Interface

```bash
mgit [-v | --version] [-h | --help] <COMMANDS> <SUBCOMMANDS>

COMMANDS:
    init      Create an empty mgit repository or reinitialize an existing one
    add       Add file contents to the staging index
    commit    Record changes to the repository history
    status    Show the working tree status

Run 'mgit <COMMANDS> --help' for more information on a specific command.
```
## Getting Started

### Installation

To run this project locally, clone the repository and build it from the source:

### Clone the repository

```bash
git clone https://github.com/your-username/mgit.git
cd mgit
```
```bash
# Build/Run instructions (Adjust based on your project's language)
# Example for Rust: cargo build --release
# Example for Go:   go build -o mgit
```
### Quick Start

1.  Initialize a new repository:
    mgit init
2.  Stage a file:
    mgit add filename.txt
3.  Check the repository status:
    mgit status

## License

This project is licensed under the AGPLv3 License - see the LICENSE file for
details.
