pub mod command_executor;
pub mod command_helper;

///
pub struct Command {
    pub name: String,
    pub description: String,
}

impl Command {
    /// Create a new command instance.
    ///
    /// # Arguments
    ///
    /// * `name` - name of command, what will be executed in shell
    /// * `description` - A short description of what the command does
    ///
    /// # Example
    ///
    /// ```
    /// let command = Command::new("pwd", "Print the current directory:");
    /// ```
    pub fn new(name: &str, description: &str) -> Self {
        Command {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}
