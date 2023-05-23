use crate::command_executor::CommandExecutor;
use crate::database::SqliteDatabase;
use colored::*;
use custom_styling::myreedline;
use reedline::{DefaultPrompt, Reedline, Signal};
use utils::{check_chosen_command, execute_current_command};

mod command;
mod command_executor;
mod command_parser;
mod custom_styling;
mod database;
mod file_parse;
mod utils;

fn main() {
    let db = setup_database();
    load_commands_into_database(&db);

    let command_exec = CommandExecutor::new();

    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::new(
        myreedline::get_left_prompt(),
        myreedline::get_right_prompt(),
    );

    loop {
        let sig = line_editor.read_line(&prompt);

        match sig {
            Ok(Signal::Success(input)) => handle_command(&input, &db, &command_exec),
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!(
                    "{}",
                    String::from("Keyboard Interrupt!").bold().bright_red()
                );
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
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

fn handle_command(input: &str, db: &SqliteDatabase, command_executor: &CommandExecutor) {
    let command_parts: Vec<&str> = input.split_whitespace().collect();

    if command_parts.len() == 1 {
        handle_single_word_command(&command_parts[0], db, command_executor);
    } else if command_parts.len() > 1 {
        handle_multi_word_commands(&command_parts, db, command_executor, input);
    }
}

fn handle_single_word_command(
    prefix: &str,
    db: &SqliteDatabase,
    command_executor: &CommandExecutor,
) {
    let matching_commands = db
        .find_matching_commands(prefix)
        .expect("Failed to find matching commands");

    if matching_commands.is_empty() {
        println!("No matching commands found.");
    } else {
        handle_multiple_returned_command(&matching_commands, prefix, &command_executor);
    }
}

fn handle_multi_word_commands(
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
            println!("No matching commands found.");
        } else {
            handle_multiple_returned_command(&matching_commands, input, &command_executor);
        }
    }
}

fn handle_multiple_returned_command(
    matching_commands: &Vec<(String, String)>,
    input: &str,
    command_executor: &CommandExecutor,
) {
    // let menu_items =
    // for menu_item in get_command_array(matching_commands, input) {
    //     println!("{}", menu_item);
    // }

    let choice = utils::menu::handle_multiple_returned_command(matching_commands, input);
    if choice == -1 {
        return;
    }
    let (command, description) = &matching_commands[choice as usize];
    check_chosen_command(command, description, command_executor)
}
