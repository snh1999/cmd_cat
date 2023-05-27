use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

/// Returns the render configuration for styling selected options.
///
/// # Returns
///
/// The render configuration for selected options.
fn _get_selected_option_style_sheet() -> StyleSheet {
    StyleSheet::default()
        .with_fg(Color::LightGreen)
        .with_attr(Attributes::BOLD)
}

/// Returns the render configuration for styling help messages.
///
/// # Returns
///
/// The render configuration for help messages.
fn _get_help_style_sheet() -> StyleSheet {
    StyleSheet::default()
        .with_fg(Color::DarkGrey)
        .with_attr(Attributes::ITALIC)
}

/// Returns the styled prompt prefix.
///
/// # Returns
///
/// The styled prompt prefix
fn _get_prompt_prefix() -> Styled<&'static str> {
    Styled::new("").with_fg(Color::DarkRed)
}

/// Returns the render configuration for custom rendering.
///
/// # Returns
///
/// The render configuration for custom rendering with
/// 1. no prompt prefix 2. greyish help message 3. lightgreen + bold selected option
pub fn my_render_config() -> RenderConfig {
    RenderConfig::default()
        .with_prompt_prefix(_get_prompt_prefix())
        .with_help_message(_get_help_style_sheet())
        .with_selected_option(Some(_get_selected_option_style_sheet()))
}

/// Returns the render configuration for confirm prompts  with no prompt prefi.
///
/// # Returns
///
/// The render configuration for confirm prompts with no prompt prefix
pub fn confirm_render_config() -> RenderConfig {
    RenderConfig::default().with_prompt_prefix(_get_prompt_prefix())
}
