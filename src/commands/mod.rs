pub mod command_executor;
pub struct Command {
    pub name: String,
    pub description: String,
}

impl Command {
    pub fn new(name: &str, description: &str) -> Self {
        Command {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}
