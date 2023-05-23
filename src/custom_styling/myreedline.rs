use reedline::DefaultPromptSegment;

pub fn get_left_prompt() -> DefaultPromptSegment {
    DefaultPromptSegment::Basic(String::from("cmd-cat "))
}

pub fn get_right_prompt() -> DefaultPromptSegment {
    DefaultPromptSegment::WorkingDirectory
}
