use crate::commands::command_executor::execute_command;
use crate::custom_styling::color_style;
use crate::custom_styling::menu_style::confirm_render_config;
use inquire::Text;
pub mod menu;

pub fn highlight_command(command: &str, input: &str) -> String {
    let command_parts: Vec<&str> = command.split_whitespace().collect();
    let input_parts: Vec<&str> = input.split_whitespace().collect();

    color_style::format_command(command_parts, input_parts)
}

pub fn highlight_description(description: &str) -> String {
    // description.bright_green().to_string()
    description.to_string()
}

pub fn execute_current_command(command: &str, description: &str) {
    println!(
        " {} {}",
        color_style::color_green(description),
        color_style::color_light_red(command)
    );

    let confirmation = menu::get_confirmation();
    // clear_previous_line();

    if confirmation {
        execute_command(command);
    } else {
        println!()
    }
}

pub fn check_chosen_command(command: &str, description: &str) {
    let command = _replace_input_string(command);

    if command.is_empty() {
        return;
    }
    execute_current_command(&command, description)
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
        let mut input: String = String::new();

        if let Some(end_delim) = input_string[start_index..].find("}}") {
            let mut end_index = start_index + end_delim + 2;
            if end_index + 1 < input_string.len() && &input_string[end_index..end_index + 1] == "["
            {
                if let Some(end_val) = input_string[end_index + 1..].find("]") {
                    let end_val_index = end_index + 1 + end_val;
                    input = String::from(&input_string[end_index + 1..end_val_index]);
                    end_index = end_val_index + 1;
                }
            } else {
                let word = &input_string[start_index + 2..end_index - 2];
                let replaced_word = replace_word(word);

                input = _get_user_input(format!("Enter '{}': ", replaced_word));
                clear_previous_line();
                if input.is_empty() {
                    return input;
                }
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

fn replace_word(word: &str) -> String {
    let text = word.replace("_", " ").replace("|", " or ");
    color_style::color_light_cyan(&text)
}

fn _get_user_input(message: String) -> String {
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
