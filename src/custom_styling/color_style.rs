use termion::color::{self, Color};
use termion::style;

/// Colorizes the given text with the specified text color.
///
/// # Arguments
///
/// * `text` - The text to colorize.
/// * `text_color` - The color to apply to the text.
///
/// # Returns
///
/// The colorized text as a `String`.
pub fn colorize_text<T: Color>(text: &str, text_color: T) -> String {
    format!(
        "{color}{text}{reset}",
        color = color::Fg(text_color),
        reset = color::Fg(color::Reset)
    )
}

pub fn color_green(text: &str) -> String {
    colorize_text(text, color::LightGreen)
}

pub fn color_light_red(text: &str) -> String {
    colorize_text(text, color::LightRed)
}

pub fn color_light_cyan(text: &str) -> String {
    colorize_text(text, color::LightCyan)
}

/// Makes the given text bold.
///
/// # Arguments
///
/// * `text` - The text to make bold.
///
/// # Returns
///
/// The bold text as a `String`.
pub fn bold_text(text: &str) -> String {
    format!(
        "{bold}{text}{reset}",
        bold = style::Bold,
        reset = style::Reset
    )
}

/// Formats the command by applying colorization based on the input parts.
/// Red for exactly matching words
/// LightBlue for partially matching words
/// LightCyan for words not present in input
///
/// # Arguments
///
/// * `command_parts` - The parts of the command.
/// * `input_parts` - The parts of the input.
///
/// # Returns
///
/// The formatted command as a `String`.
pub fn format_command(command_parts: Vec<&str>, input_parts: Vec<&str>) -> String {
    let colorfn = |index: usize| match input_parts.get(index) {
        Some(_) if index < command_parts.len() => {
            if index < input_parts.len() && input_parts[index] == command_parts[index] {
                colorize_text(command_parts[index], color::Red)
            } else {
                colorize_text(command_parts[index], color::LightBlue)
            }
        }
        _ => colorize_text(command_parts[index], color::LightCyan),
    };

    command_parts
        .iter()
        .enumerate()
        .map(|(i, _)| colorfn(i))
        .collect::<Vec<String>>()
        .join(" ")
}
