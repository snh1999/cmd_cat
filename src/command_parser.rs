pub struct CommandParser {}

impl CommandParser {
    pub fn new() -> Self {
        CommandParser {}
    }

    pub fn parse_command<'a>(&self, input: &'a str) -> Vec<&'a str> {
        input.split_whitespace().collect()
    }
}
