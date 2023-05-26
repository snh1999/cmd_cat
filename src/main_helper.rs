use crate::commands::command_executor::CommandExecutor;
use crate::database::SqliteDatabase;
use crate::utils;
use crate::utils::{check_chosen_command, execute_current_command};

pub fn setup_database() -> SqliteDatabase {
    SqliteDatabase::new().expect("Failed to create database")
}

pub fn handle_command(input: &str, db: &SqliteDatabase, command_executor: &CommandExecutor) {
    let command_parts: Vec<&str> = input.split_whitespace().collect();

    if command_parts.len() == 1 {
        handle_single_word_command(&command_parts[0], db, command_executor);
    } else if command_parts.len() > 1 {
        handle_multi_word_commands(&command_parts, db, command_executor, input);
    }
}

pub fn handle_single_word_command(
    prefix: &str,
    db: &SqliteDatabase,
    command_executor: &CommandExecutor,
) {
    let matching_commands = db
        .find_matching_commands(prefix)
        .expect("Failed to find matching commands");

    if matching_commands.is_empty() {
        search_in_database(prefix, db, command_executor);
    } else {
        handle_multiple_returned_command(&matching_commands, prefix, &command_executor);
    }
}

pub fn handle_multi_word_commands(
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
        execute_current_command(&command, &description, command_executor);
    } else {
        let matching_commands = db
            .find_matching_commands(&command)
            .expect("Failed to find matching commands");

        if matching_commands.is_empty() {
            handle_progressive_search(command_parts, db, command_executor, input)
        } else {
            handle_multiple_returned_command(&matching_commands, input, &command_executor);
        }
    }
}

pub fn handle_multiple_returned_command(
    matching_commands: &Vec<(String, String)>,
    input: &str,
    command_executor: &CommandExecutor,
) {
    let choice = utils::menu::handle_multiple_returned_command(matching_commands, input);
    if choice == -1 {
        return;
    }
    let (command, description) = &matching_commands[choice as usize];
    check_chosen_command(command, description, command_executor)
}

pub fn handle_progressive_search(
    command_parts: &[&str],
    db: &SqliteDatabase,
    command_executor: &CommandExecutor,
    input: &str,
) {
    let matching_commands = db
        .find_matching_commands(&command_parts[0])
        .expect("Failed to find matching commands");
    let matching_commands_copy = matching_commands.clone();

    let mut command_words = matching_commands_copy
        .iter()
        .enumerate()
        .map(|(index, (command, _))| (index, command.split_whitespace().collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    for (i, word) in command_parts.iter().enumerate().skip(1) {
        let mut new_matching_commands;

        new_matching_commands = command_words
            .clone()
            .into_iter()
            .filter(|(_, command_words)| {
                if !(command_words.len() < i + 1) {
                    if command_words[i] == *word {
                        return true;
                    }
                }
                return false;
            })
            .collect::<Vec<_>>();

        if new_matching_commands.is_empty() {
            new_matching_commands = command_words
                .clone()
                .into_iter()
                .filter(|(_, command_words)| {
                    if !(command_words.len() < i + 1) {
                        if command_words[i].starts_with("{{") {
                            return true;
                        }
                    }
                    return false;
                })
                .collect::<Vec<_>>();
        }

        if new_matching_commands.is_empty() {
            break;
        }
        command_words = new_matching_commands
    }

    if command_words.is_empty() {
        search_in_database(input, db, command_executor);
    } else {
        let matching_commands = filter_matching_commands(command_words, matching_commands);

        if matching_commands.len() == 1 {
            let (command, description) = &matching_commands[0];
            execute_current_command(&command, &description, command_executor);
        } else {
            handle_multiple_returned_command(&matching_commands, input, command_executor);
        }
    }
}

pub fn filter_matching_commands(
    command_words: Vec<(usize, Vec<&str>)>,
    matching_commands: Vec<(String, String)>,
) -> Vec<(String, String)> {
    let mut new_matching_commands = Vec::new();
    for (i, _) in command_words {
        let (command, description) = &matching_commands[i];
        new_matching_commands.push((command.to_owned(), description.to_owned()));
    }
    new_matching_commands
}

pub fn search_in_database(input: &str, db: &SqliteDatabase, command_executor: &CommandExecutor) {
    let matching_commands = match db.search_commands(input) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };
    if matching_commands.is_empty() {
        println!("No matches found.");
    } else {
        handle_multiple_returned_command(&matching_commands, "", command_executor);
    }
}
