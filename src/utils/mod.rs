use crate::command_executor::CommandExecutor;
use crate::custom_styling::menu_style::confirm_render_config;
use colored::*;
use inquire::Text;
pub mod menu;

pub fn highlight_command(command: &str, input: &str) -> String {
    let command_parts: Vec<&str> = command.split_whitespace().collect();
    let input_parts: Vec<&str> = input.split_whitespace().collect();

    let color = |index: usize| match input_parts.get(index) {
        Some(_) if index < command_parts.len() => {
            if index < input_parts.len() && input_parts[index] == command_parts[index] {
                Color::BrightRed
            } else {
                Color::Red
            }
        }
        _ => Color::BrightCyan,
    };

    let highlighted_command = command_parts
        .iter()
        .enumerate()
        .map(|(i, part)| part.color(color(i)).to_string())
        .collect::<Vec<String>>()
        .join(" ");

    highlighted_command
}

pub fn highlight_description(description: &str) -> String {
    // description.bright_green().to_string()
    description.to_string()
}

pub fn execute_current_command(
    command: &str,
    description: &str,
    command_executor: &CommandExecutor,
) {
    println!(" {} {}", description.bright_green(), command.bright_red());

    let confirmation = menu::get_confirmation();
    // clear_previous_line();

    if confirmation {
        command_executor.execute_command(command);
    } else {
        println!()
    }
}

pub fn check_chosen_command(command: &str, description: &str, command_executor: &CommandExecutor) {
    let command = _replace_input_string(command);

    if command.is_empty() {
        return;
    }
    execute_current_command(&command, description, command_executor)
}

fn clear_selection_lines() {
    clear_previous_line(); // matched command line
    clear_previous_line(); // description
    clear_previous_line(); // actual command
}

fn _replace_input_string(input_string: &str) -> String {
    let mut replaced_string = String::new();
    let mut start = 0;

    while let Some(start_delim) = input_string[start..].find("{{") {
        let start_index = start + start_delim;

        if let Some(end_delim) = input_string[start_index..].find("}}") {
            let end_index = start_index + end_delim + 2;
            let word = &input_string[start_index + 2..end_index - 2];
            let replaced_word = word.replace("_", " ").replace("|", " or ").bright_cyan();

            let input = _get_user_input(format!("Enter '{}': ", replaced_word));
            clear_previous_line();
            if input.is_empty() {
                return input;
            }
            replaced_string += &input_string[start..start_index];
            replaced_string += &input;

            start = end_index;
        } else {
            replaced_string += &input_string[start..];
            break;
        }
    }
    clear_selection_lines();

    replaced_string += &input_string[start..]; // Append the remaining portion of the input string
    replaced_string
}

fn _get_user_input(message: String) -> String {
    // todo - clear line
    let name = Text::new(&message)
        .with_render_config(confirm_render_config())
        .prompt();

    match name {
        Ok(name) => name,
        Err(_) => "".to_owned(),
    }
}

pub fn clear_previous_line() {
    print!("\x1B[1A\x1B[K"); // Move up one line and clear the line
}
