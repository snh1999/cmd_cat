use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

fn _get_selected_option_style_sheet() -> StyleSheet {
    StyleSheet::default()
        .with_fg(Color::LightGreen)
        .with_attr(Attributes::BOLD)
}

fn _get_help_style_sheet() -> StyleSheet {
    StyleSheet::default()
        .with_fg(Color::DarkGrey)
        .with_attr(Attributes::ITALIC)
}

fn _get_prompt_prefix() -> Styled<&'static str> {
    Styled::new("").with_fg(Color::DarkRed)
}

pub fn my_render_config() -> RenderConfig {
    RenderConfig::default()
        .with_prompt_prefix(_get_prompt_prefix())
        .with_help_message(_get_help_style_sheet())
        .with_selected_option(Some(_get_selected_option_style_sheet()))
}

pub fn confirm_render_config() -> RenderConfig {
    RenderConfig::default().with_prompt_prefix(_get_prompt_prefix())
}
