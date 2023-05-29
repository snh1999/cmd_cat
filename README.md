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
cmd_cat --update
```

## Installation

### Build from source

To install CMD Cat, you need to have Rust and Cargo installed. Then, you can build the project from source:

```bash
cargo build --release
```

NOTICE: for first time usage, you will need to update the database by running `cmd_cat --update` which will take around 3MB of internet and 2MB of disk space. If you move the binary, either move the `commands.db` as well or run update again in new location.

### Releases

To download the application, please visit the releases page. You will find two options available: an AppImage and a Linux x86_64 binary.

Make sure to also download the `commands.db` file, which is available in releases (also packaged with the AppImage - cmd-cat.appimage.tar.xz, just extracting the folder would do). It's important to keep the executable file in the same folder as the `commands.db` file.

Alternatively, you can run `./cmd-cat --update`(cmd-cat is the downloaded filename) after downloading the binary. Ensure that you launch the command from the suitable folder where you have placed the downloaded file. This command will create the database and place it same folder as binary download. Please note that you should have Git installed on your system for this operation.

## Dependencies

CMD Cat depends on the following external crates:

-   rusqlite: 0.29.0
-   rustyline = 11.0.0
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
