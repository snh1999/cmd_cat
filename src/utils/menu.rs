use crate::custom_styling::color_style;
use crate::custom_styling::menu_style::{confirm_render_config, my_render_config};
use crate::utils::{clear_previous_line, highlight_command, highlight_description};

use inquire::{Confirm, Select};

/// Gets the confirmation from the user to execute the command.
///
/// # Returns
///
/// `true` if the user confirms execution, `false` otherwise.
pub fn get_confirmation() -> bool {
    get_custom_confirmation("Do you want to execute the command?")
}

/// Gets a custom confirmation from the user with the specified prompt text.
///
/// # Arguments
///
/// * `prompt_text` - The text to display as the confirmation prompt.
///
/// # Returns
///
/// `true` if the user confirms, `false` otherwise.
pub fn get_custom_confirmation(prompt_text: &str) -> bool {
    let response = Confirm::new(prompt_text)
        .with_default(true)
        .with_render_config(confirm_render_config())
        .prompt();
    clear_previous_line();

    let response = match response {
        Ok(data) => data,
        Err(_) => false,
    };

    response
}

/// Gets the array of formatted command menu items.
///
/// # Arguments
///
/// * `matching_commands` - The vector of matching commands.
/// * `input` - The input string to match against.
///
/// # Returns
///
/// A vector of formatted command menu items.
fn get_command_array(matching_commands: &Vec<(String, String)>, input: &str) -> Vec<String> {
    let mut menu_items: Vec<String> = Vec::new();
    for (command, description) in matching_commands {
        let colored_command = highlight_command(&command, input);
        let colored_description = highlight_description(&description);

        let menu_item = format!("{}\n  {}", colored_description, colored_command);
        menu_items.push(menu_item);
    }
    menu_items
}

/// Displays the commands menu and returns the selected index.
///
/// # Arguments
///
/// * `menu_items` - The vector of formatted command menu items.
///
/// # Returns
///
/// The selected index of the command menu.
fn commands_menu(menu_items: Vec<String>) -> Result<usize, i32> {
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
                color_style::bold_text("Selected Command"),
                color_style::color_green(description),
                command
            );
            return Ok(data.index);
        }
        Err(_) => Err(-1),
    }
}

/// Handles multiple returned commands by displaying a menu and returning the selected index.
///
/// # Arguments
///
/// * `matching_commands` - The vector of matching commands.
/// * `input` - The input string to match against.
///
/// # Returns
///
/// The selected index of the command menu.
pub fn handle_multiple_returned_command(
    matching_commands: &Vec<(String, String)>,
    input: &str,
) -> Result<usize, i32> {
    let menu_items = get_command_array(matching_commands, input);
    let response = commands_menu(menu_items);
    response
}
