> [!WARNING]
> I wrote this project strictly to explain and understand how Git works under the hood. It would be unwise to use this to version control your actual projects, but it works for learning purposes.
> It is an **incomplete, half-finished prototype**. It only implements basic operations (`init`, `add`, `status`, `commit`) and will not be completed. It has served its learning purpose and is now **retired, read-only, and no longer maintained**. The codebase is kept here as a static reference.
# mgit

(A version control system designed to track file changes)

**Current Version:** v0.0.1 (Pre-alpha)



### Features & Status

*   **Repository Initialization** (`mgit init`): Creates the internal database structure to track your files.
*   **Staging** (`mgit add`): Adds file contents to the staging index.
*   **Status Tracking** (`mgit status`): Shows differences between the working directory and the staging area.
*   **Recording Changes** (`mgit commit`): can now record and maintain a linear history.

### Why I'm Building This

This project is a personal learning exercise driven by two main goals:

*   **Understanding Git:** The best way to understand how a tool works is to build it from scratch.
   Implementing the core mechanics of Git helps demystify how it manages, tracks, and stores data.
*   **Learning Rust:** I wanted a practical, hands-on project to practice and deepen my understanding of Rust. A
   version control system provides a great way to explore Rust's safety guarantees, ownership model, and file system APIs.

### Command Line Interface

```text
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
git clone https://github.com/debuggerdragon311/mgit.git
cd mgit
```

### Build/Run instructions
```bash
cargo build --release
```

## License

This project is licensed under the AGPLv3 License - see the LICENSE file for
details.
