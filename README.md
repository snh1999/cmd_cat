# CMD Cat

CMD Cat is a command-line tool for searching and executing commands. It provides a REPL (Read-Eval-Print Loop) interface where you can enter commands and get results.

## Features

-   Search and execute commands
-   REPL interface for interactive usage
-   Database for command storage and retrieval

## Usage

To use CMD Cat, you can run the binary directly or use the REPL interface.

### Command Line Arguments

If you provide command-line arguments when running CMD Cat, it will execute the command specified.

Example:

```bash
 cmd_cat meow <search-term>
 cmd_cat <command>
```

### REPL Interface

If no command-line arguments are provided, CMD Cat starts in REPL mode. You can enter commands interactively and get results.

Example:

```bash
cmd_cat
cmd-cat> <command>
cmd-cat> meow <search-term>
```

### Updating the Database

CMD Cat relies on a database of commands for searching and execution. You can update the database by running the following command:

```bash
cmd_cat
cmd-cat> <command>
cmd-cat> meow <search-term>
```

## Installation

### Build from source

To install CMD Cat, you need to have Rust and Cargo installed. Then, you can build the project from source:

```bash
cargo build --release
```

### Releases

Will be added later as soon as Testing is done.

## Dependencies

CMD Cat depends on the following external crates:

-   rusqlite: 0.29.0
-   reedline: 0.19.1
-   inquire: 0.6.2
-   termion: 2.0.1

## Planned Improvements

-   Enhance search functionality for better command discovery, if possible Integrate NLP-based search system
-   Improve efficiency by minimizing dependencies (if possible)
-   Incorporate man pages for comprehensive command information
-   Autocompletion inspired by Node.js REPL

## Database Efficiency

CMD Cat downloads from [tldr-page](https://github.com/snh1999/tldr-page) for improved efficiency while update. This repository trims down the original tldr pages git commit (26MB) to ensure faster download access to command information. The repository will be kept updated to provide the latest command references.

## Contributing

Contributions to CMD Cat are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

CMD Cat is released under the MIT License. See the [LICENSE](LICENSE) file for details.
