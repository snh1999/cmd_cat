use commands::command_executor::execute_command;
use commands::command_helper::*;
use custom_styling::color_style;
use custom_styling::myreedline;
use database::file_parse::_clean_update_database;
use database::SqliteDatabase;
use reedline::{DefaultPrompt, Reedline, Signal};
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
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::new(
        myreedline::get_left_prompt(),
        myreedline::get_right_prompt(),
    );

    loop {
        let sig = line_editor.read_line(&prompt);

        match sig {
            Ok(Signal::Success(input)) => handle_input(&input, &db),
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!(
                    "{}",
                    color_style::bold_text(&color_style::color_light_red("Keyboard Interrupt!"))
                );
                break;
            }
            x => {
                println!("Event: {:?}", x);
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
