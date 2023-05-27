use crate::commands::command_executor::execute_command;
use crate::database::SqliteDatabase;
use crate::utils::menu::{self, get_custom_confirmation};
use crate::utils::{check_chosen_command, execute_current_command};

/// Get a new instance of SqliteDatabase, basically separates the error handling side of things.
///
/// # Returns
///
/// The initialized `SqliteDatabase` instance.
pub fn setup_database() -> SqliteDatabase {
    SqliteDatabase::new().expect("Failed to create database")
}

/// A helper function to handle error while getting the matching commands based on the given prefix.
///
/// # Arguments
///
/// * `db` - The database instance.
/// * `prefix` - The prefix to search for.
///
/// # Returns
///
/// A vector of matching command names and descriptions.
fn get_matching_commands(db: &SqliteDatabase, prefix: &str) -> Vec<(String, String)> {
    db.find_matching_commands(prefix)
        .expect("Failed to find matching commands")
}

/// Handle the input command by deligating it to specific funtions.
///
/// # Arguments
///
/// * `input` - The input command.
/// * `db` - The database instance.
pub fn handle_command(input: &str, db: &SqliteDatabase) {
    let command_parts: Vec<&str> = input.split_whitespace().collect();

    if command_parts.len() == 1 {
        handle_single_word_command(&command_parts[0], db);
    } else if command_parts.len() > 1 {
        handle_multi_word_commands(&command_parts, db, input);
    }
}

/// Handle a single-word command by searching for all the commands starting with that input/prefix.
/// Performs a Database search if no match found in commands, or shows a selectable menu with all of the commands
///
/// # Arguments
///
/// * `prefix` - The command prefix.
/// * `db` - The database instance.
pub fn handle_single_word_command(prefix: &str, db: &SqliteDatabase) {
    let matching_commands = get_matching_commands(db, prefix);

    if matching_commands.is_empty() {
        search_in_database(prefix, db);
    } else {
        handle_multiple_returned_command(&matching_commands, prefix);
    }
}

/// Handle command containing more than one words.
/// If we find some exact match, we allow to execute the program(1st try)
/// If 1 fails, it tries to make command as a prefix and view a selectable menu
/// If 2 fails, placeholders are considered for a match
///
/// # Arguments
///
/// * `command_parts` - The parts of the command.
/// * `db` - The database instance.
/// * `input` - The input command.
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

/// Handle commands with multiple matches, by prompting a selectable menu
/// If some choice is supplied, it is send for placeholder verification
///
/// # Arguments
///
/// * `matching_commands` - The matching command names and descriptions.
/// * `input` - The input command.
pub fn handle_multiple_returned_command(matching_commands: &Vec<(String, String)>, input: &str) {
    let choice = menu::handle_multiple_returned_command(matching_commands, input);
    match choice {
        Ok(choice) => {
            let (command, description) = &matching_commands[choice as usize];
            check_chosen_command(command, description);
        }
        Err(_) => return,
    }
}

/// Handle progressive search for commands by going by matching each word or placeholder
/// Commands matching the first word allowed for further check
/// Each word of string has to have exact match or need a placeholder string at that position to avoid elimination
/// In case we the string is empty at some stage, send back the matching commands the round before it
///
/// # Arguments
///
/// * `command_parts` - The parts of the command.
/// * `db` - The database instance.
/// * `input` - The input command
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

/// Search for a command in the database (Matching Command or Description)
/// Matches are shown as a menu
/// If No matches found, user is prompted for further actions.
///
/// # Arguments
///
/// * `input` - The input command.
/// * `db` - The database instance.
pub fn search_in_database(input: &str, db: &SqliteDatabase) {
    let matching_commands = match db.search_commands(input) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };
    if matching_commands.is_empty() {
        println!("No matches found.");
        let response = get_custom_confirmation(
            "Do you still want to proceed to execution of the command? It might cause Fatal error.",
        );
        if response {
            execute_command(input)
        }
    } else {
        handle_multiple_returned_command(&matching_commands, "");
    }
}

/// Helper function for handle_progressive_search Convert matching vector of commands(tuple) to a vector of words array.
///
/// # Arguments
///
/// * `matching_commands` - The matching command names and descriptions.
///
/// # Returns
///
/// A vector of word arrays of commands.
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

/// Helper function for handle_progressive_search Filter command words by exact match
/// (words at ith position of commands matching word at position i of input string are allowed)
///
/// # Arguments
///
/// * `command_words` - The command words from search.
/// * `word` - The word to match (from input).
/// * `word_index` - The index of the word (position) in the input command string.
///
/// # Returns
///
/// A vector of matching command words.
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

/// Helper function for handle_progressive_search Filter command words by exact match
/// (placeholder exist at ith position of commands matching word)
///
/// # Arguments
///
/// * `command_words` - The command words from search.
/// * `word` - The word to match (from input).
/// * `word_index` - The index of the word (position) in the input command string.
///
/// # Returns
///
/// A vector of matching command words with updated placeholders.

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

/// Filter matching commands based on surviving command words (Basically it extract the descrption)
///
/// # Arguments
///
/// * `command_words` - The command words surviving elemination
/// * `matching_commands` - The matching command names and descriptions.
///
/// # Returns
///
/// A vector of matching command names and descriptions.
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
