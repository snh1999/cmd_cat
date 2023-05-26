use crate::command::Command;
use crate::command_executor::CommandExecutor;
use crate::command_parser::CommandParser;
use crate::database::SqliteDatabase;
use colored::*;
use std::io::{self, Write};

mod command;
mod command_executor;
mod command_parser;
mod database;
mod file_parse;

fn main() {
    let db = setup_database();

    load_commands_into_database(&db);

    let command_executor = CommandExecutor::new();
    let command_parser = CommandParser::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let input = read_input();

        if input.is_empty() {
            continue;
        }

        handle_command(&input, &db, &command_executor, &command_parser);
    }
}

fn setup_database() -> SqliteDatabase {
    SqliteDatabase::new().expect("Failed to create database")
}

fn load_commands_into_database(db: &SqliteDatabase) {
    db.clear().expect("Failed to clean database");
    file_parse::insert_commands_from_file("./yay.md", db);
    file_parse::insert_commands_from_file("./cd.md", db);
    file_parse::insert_commands_from_file("./pwd.md", db);
    // Add more commands to the database
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn handle_command(
    input: &str,
    db: &SqliteDatabase,
    command_executor: &CommandExecutor,
    command_parser: &CommandParser,
) {
    let command_parts = command_parser.parse_command(input);

    if command_parts.len() == 1 {
        handle_single_command(&command_parts[0], db);
    } else if command_parts.len() > 1 {
        handle_multiple_commands(&command_parts, db, command_executor, input);
    }
}

fn handle_single_command(prefix: &str, db: &SqliteDatabase) {
    let matching_commands = db
        .find_matching_commands(prefix)
        .expect("Failed to find matching commands");

    if matching_commands.is_empty() {
        println!("No matching commands found.");
    } else {
        println!("Matching commands:");
        for (command, description) in matching_commands {
            let colored_command = highlight_command(&command, prefix);
            println!("- {}", description.green());
            println!("  {}", colored_command);
        }
    }
}

fn handle_multiple_commands(
    command_parts: &[&str],
    db: &SqliteDatabase,
    command_executor: &CommandExecutor,
    input: &str,
) {
    let command = command_parts.join(" ");

    if let Some(description) = db
        .get_command_description(&command)
        .expect("Failed to get command description")
    {
        println!("{}", description.green());
        let confirmation = get_confirmation();

        if confirmation.is_empty() || confirmation == "y" {
            clear_previous_line();
            command_executor.execute_command(&command);
        }
    } else {
        let matching_commands = db
            .find_matching_commands(&command)
            .expect("Failed to find matching commands");

        if matching_commands.is_empty() {
            println!("No matching commands found.");
        } else {
            println!("Matching commands:");
            for (command, description) in matching_commands {
                let colored_command = highlight_command(&command, input);
                println!("- {}", description.green());
                println!("  {}", colored_command);
            }
        }
    }
}

fn highlight_command(command: &str, input: &str) -> String {
    let command_parts: Vec<&str> = command.split_whitespace().collect();
    let input_parts: Vec<&str> = input.split_whitespace().collect();

    let color = |index: usize| match input_parts.get(index) {
        Some(_) if index < command_parts.len() => {
            if index < input_parts.len() && input_parts[index] == command_parts[index] {
                Color::Red
            } else {
                Color::Cyan
            }
        }
        _ => Color::Cyan,
    };

    let highlighted_command = command_parts
        .iter()
        .enumerate()
        .map(|(i, part)| part.color(color(i)).to_string())
        .collect::<Vec<String>>()
        .join(" ");

    highlighted_command
}

// Helper functions

fn get_confirmation() -> String {
    print!("Do you want to execute the command? (y/n) ");
    io::stdout().flush().unwrap();

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).unwrap();

    confirmation.trim().to_owned()
}

fn clear_previous_line() {
    print!("\x1B[1A\x1B[K"); // Move up one line and clear the line
}
