use crate::custom_styling::menu_style::{confirm_render_config, my_render_config};
use crate::utils::{clear_previous_line, highlight_command, highlight_description};
use colored::Colorize;
use inquire::{Confirm, Select};

pub fn get_confirmation() -> bool {
    get_custom_confirmation("Do you want to execute the command?")
}

pub fn get_custom_confirmation(prompt_text: &str) -> bool {
    let response = Confirm::new(prompt_text)
        .with_default(true)
        .with_render_config(confirm_render_config())
        .prompt();
    let response = match response {
        Ok(data) => data,
        Err(_) => false,
    };

    clear_previous_line();
    response
}

pub fn get_command_array(matching_commands: &Vec<(String, String)>, input: &str) -> Vec<String> {
    let mut menu_items: Vec<String> = Vec::new();
    for (command, description) in matching_commands {
        let colored_command = highlight_command(&command, input);
        let colored_description = highlight_description(&description);

        let menu_item = format!("{}\n  {}", colored_description, colored_command);
        menu_items.push(menu_item);
    }
    menu_items
}

fn _commands_menu(menu_items: Vec<String>) -> i32 {
    let response = Select::new("Matching commands", menu_items)
        .with_render_config(my_render_config())
        .with_help_message(
            "↑↓ to move, enter to select, type to filter, press Esc/(Ctrl+C) to cancel",
        )
        .with_formatter(&|_| "".to_string())
        .raw_prompt();
    clear_previous_line();

    match response {
        Ok(data) => {
            // we need to print this to view the effect of clear line above
            let (description, command) = data.value.split_once("\n").unwrap();
            println!(
                "  {}:\n  {}\n{}",
                "Selected Command".bold(),
                description.bright_green(),
                command
            );
            return data.index as i32;
        }
        Err(_) => -1,
    }
}

pub fn handle_multiple_returned_command(
    matching_commands: &Vec<(String, String)>,
    input: &str,
) -> i32 {
    let menu_items = get_command_array(matching_commands, input);
    let response = _commands_menu(menu_items);
    response
}
