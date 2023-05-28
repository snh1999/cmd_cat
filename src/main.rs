use commands::command_executor::execute_command;
use commands::command_helper::*;

use custom_styling::color_style;

use database::file_parse::_clean_update_database;
use database::SqliteDatabase;

use custom_styling::color_style::style_prompt_text;

use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use std::{env, fs, path::Path};

mod commands;
mod custom_styling;
mod database;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let db = setup_database();
    if args.len() > 1 {
        // Command line arguments provided
        let input = args[1..].join(" ");
        if input == "--update" {
            update_database(&db);
        } else {
            handle_input(&input, &db);
        }
    } else {
        // No command line arguments, start REPL
        start_repl(db);
    }
}

fn start_repl(db: SqliteDatabase) {
    let mut rl = DefaultEditor::new().unwrap();
    rl.set_max_history_size(100).unwrap();
    loop {
        let input = rl.readline(&style_prompt_text("cmd-cat> "));
        match input {
            Ok(input) => {
                rl.add_history_entry(input.as_str()).unwrap();
                handle_input(&input, &db);
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!(
                    "{}",
                    color_style::bold_text(&color_style::color_light_red("Keyboard Interrupt!"))
                );
                break;
            }
            Err(err) => {
                println!("{err}");
                break;
            }
        }
    }
}

fn handle_input(input: &str, db: &SqliteDatabase) {
    input.split_once(" ").map(|(first_word, rest_string)| {
        if first_word == "meow" {
            search_in_database(rest_string, db);
            return;
        }
    });
    handle_command(input, db)
}

fn update_database(db: &SqliteDatabase) {
    let folder_path = "./tldr-page";
    if Path::new(folder_path).is_dir() {
        fs::remove_dir_all("./tldr-page").unwrap();
    }
    println!(
        "{}",
        color_style::color_light_red("Make sure you have an internet connection and git installed in your System. This operation will download the necessary files in your system")
    );
    let response = utils::menu::get_custom_confirmation("Do you want to proceed?");
    if !response {
        return;
    }
    execute_command("git clone https://github.com/snh1999/tldr-page.git");
    fs::remove_dir_all("./tldr-page/.git").unwrap();
    _clean_update_database(db, folder_path);
    fs::remove_dir_all("./tldr-page").unwrap();
}
