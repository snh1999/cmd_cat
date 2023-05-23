use std::process::{exit, Command};

pub struct CommandExecutor {}

impl CommandExecutor {
    pub fn new() -> Self {
        CommandExecutor {}
    }

    pub fn execute_command(&self, command: &str) {
        print!(">>");
        let mut parts = command.split_whitespace();
        if let Some(command_name) = parts.next() {
            let mut process = Command::new(command_name);
            process.args(parts);

            let output = match process.output() {
                Ok(output) => output,
                Err(error) => {
                    eprintln!("Failed to execute command '{}': {}", command, error);
                    exit(1);
                }
            };

            if !output.stdout.is_empty() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
            }

            if !output.stderr.is_empty() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Command error:\n{}", stderr);
            }
        } else {
            println!("Invalid command");
        }
    }
}
