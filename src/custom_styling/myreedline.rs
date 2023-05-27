use reedline::DefaultPromptSegment;

/// Returns the reedline left prompt segment.
///
/// # Returns
///
/// The left prompt segment of text cmd-cat.
pub fn get_left_prompt() -> DefaultPromptSegment {
    DefaultPromptSegment::Basic(String::from("cmd-cat "))
}

/// Returns the right prompt segment.
///
/// # Returns
///
/// The right prompt segment showing current working directory
pub fn get_right_prompt() -> DefaultPromptSegment {
    DefaultPromptSegment::WorkingDirectory
}
