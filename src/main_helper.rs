use crate::commands::command_executor::execute_command;
use crate::database::SqliteDatabase;
use crate::utils::menu::{self, get_custom_confirmation};
use crate::utils::{check_chosen_command, execute_current_command};

pub fn setup_database() -> SqliteDatabase {
    SqliteDatabase::new().expect("Failed to create database")
}

fn get_matching_commands(db: &SqliteDatabase, prefix: &str) -> Vec<(String, String)> {
    db.find_matching_commands(prefix)
        .expect("Failed to find matching commands")
}

pub fn handle_command(input: &str, db: &SqliteDatabase) {
    let command_parts: Vec<&str> = input.split_whitespace().collect();

    if command_parts.len() == 1 {
        handle_single_word_command(&command_parts[0], db);
    } else if command_parts.len() > 1 {
        handle_multi_word_commands(&command_parts, db, input);
    }
}

pub fn handle_single_word_command(prefix: &str, db: &SqliteDatabase) {
    let matching_commands = get_matching_commands(db, prefix);

    if matching_commands.is_empty() {
        search_in_database(prefix, db);
    } else {
        handle_multiple_returned_command(&matching_commands, prefix);
    }
}

pub fn handle_multi_word_commands(command_parts: &[&str], db: &SqliteDatabase, input: &str) {
    let command = command_parts.join(" ");

    if let Some(description) = db
        .get_command_description(&command)
        .expect("Failed to get command description")
    {
        execute_current_command(&command, &description);
    } else {
        let matching_commands = get_matching_commands(db, &command);

        if matching_commands.is_empty() {
            handle_progressive_search(command_parts, db, input)
        } else {
            handle_multiple_returned_command(&matching_commands, input);
        }
    }
}

pub fn handle_multiple_returned_command(matching_commands: &Vec<(String, String)>, input: &str) {
    let choice = menu::handle_multiple_returned_command(matching_commands, input);
    if choice == -1 {
        return;
    }
    let (command, description) = &matching_commands[choice as usize];
    check_chosen_command(command, description)
}

pub fn handle_progressive_search(command_parts: &[&str], db: &SqliteDatabase, input: &str) {
    let matching_commands = get_matching_commands(db, &command_parts[0]);

    let mut command_words = string_to_words_arr(&matching_commands);

    for (word_index, word) in command_parts.iter().enumerate().skip(1) {
        let mut new_matching_commands;

        new_matching_commands = filter_by_exact_match(&command_words, word, word_index);

        if new_matching_commands.is_empty() {
            new_matching_commands = filter_by_placeholder_match(&command_words, word, word_index);
        }

        if new_matching_commands.is_empty() {
            break;
        }
        command_words = new_matching_commands;
    }

    if command_words.is_empty() {
        search_in_database(input, db);
    } else {
        let matching_commands = filter_matching_commands(command_words, matching_commands);

        if matching_commands.len() == 1 {
            let (command, description) = &matching_commands[0];
            execute_current_command(&command, &description);
        } else {
            handle_multiple_returned_command(&matching_commands, input);
        }
    }
}

fn string_to_words_arr(matching_commands: &Vec<(String, String)>) -> Vec<(usize, Vec<String>)> {
    matching_commands
        .clone()
        .iter()
        .enumerate()
        .map(|(index, (command, _))| {
            (
                index,
                command
                    .split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        })
        .collect::<Vec<_>>()
}

pub fn search_in_database(input: &str, db: &SqliteDatabase) {
    let matching_commands = match db.search_commands(input) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };
    if matching_commands.is_empty() {
        println!("No matches found.");
        let response =
            get_custom_confirmation("Do you still want to try to execute the input command");
        if response {
            execute_command(input)
        }
    } else {
        handle_multiple_returned_command(&matching_commands, "");
    }
}

fn filter_by_exact_match(
    command_words: &Vec<(usize, Vec<String>)>,
    word: &str,
    word_index: usize,
) -> Vec<(usize, Vec<String>)> {
    command_words
        .clone()
        .into_iter()
        .filter(|(_, command_word)| {
            if !(command_word.len() < word_index + 1) {
                if command_word[word_index] == word {
                    return true;
                }
            }
            return false;
        })
        .collect::<Vec<_>>()
}

fn filter_by_placeholder_match(
    command_words: &Vec<(usize, Vec<String>)>,
    word: &str,
    word_index: usize,
) -> Vec<(usize, Vec<String>)> {
    command_words
        .clone()
        .into_iter()
        .filter(|(_, command_word)| {
            !(command_word.len() < word_index + 1) && command_word[word_index].starts_with("{{")
        })
        .map(|(command, mut command_word)| {
            command_word[word_index] = format!("{}[{}]", command_word[word_index], word);
            (command, command_word)
        })
        .collect::<Vec<_>>()
}

pub fn filter_matching_commands(
    command_words: Vec<(usize, Vec<String>)>,
    matching_commands: Vec<(String, String)>,
) -> Vec<(String, String)> {
    let mut new_matching_commands = Vec::new();
    for (i, command) in command_words {
        let (_, description) = &matching_commands[i];
        new_matching_commands.push((command.join(" ").to_owned(), description.to_owned()));
    }
    new_matching_commands
}
