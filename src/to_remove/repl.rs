use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut database: HashMap<String, String> = HashMap::new();
    database.insert("yay".to_string(), "Yay command description".to_string());
    database.insert(
        "yay123".to_string(),
        "Yay123 command description".to_string(),
    );
    // Add more commands to the database as needed

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.contains(' ') {
            let mut exact_match = false;
            for (command, description) in &database {
                if command == input {
                    println!("Command found: {}\nDescription: {}", command, description);
                    exact_match = true;
                    break;
                }
            }

            if !exact_match {
                let matching_commands = database
                    .iter()
                    .filter(|(command, _)| command.starts_with(input))
                    .map(|(command, _)| command.clone())
                    .collect::<Vec<String>>();

                if matching_commands.is_empty() {
                    println!("No matching commands found.");
                } else {
                    println!("Matching commands:");
                    for (index, command) in matching_commands.iter().enumerate() {
                        println!("{}. {}", index + 1, command);
                    }

                    println!("Please select a command by entering its number (or 0 to exit):");

                    let mut choice = String::new();
                    io::stdin().read_line(&mut choice).unwrap();
                    let choice = choice.trim().parse::<usize>().unwrap_or(0);

                    if choice > 0 && choice <= matching_commands.len() {
                        let selected_command = &matching_commands[choice - 1];
                        if selected_command.contains("<name>") {
                            println!("Please enter a name:");
                            let mut name = String::new();
                            io::stdin().read_line(&mut name).unwrap();
                            let name = name.trim();
                            execute_command(selected_command, name);
                        } else {
                            execute_command(selected_command, "");
                        }
                    } else {
                        println!("Invalid choice.");
                    }
                }
            }
        } else {
            if let Some(description) = database.get(input) {
                println!("Command found: {}\nDescription: {}", input, description);
                execute_command(input, "");
            } else {
                println!("Invalid command: {}", input);
            }
        }
    }
}

fn execute_command(command: &str, name: &str) {
    // Implement the logic to execute the command here
    // You can use the `command` and `name` variables as needed
    println!("Executing command: {} with name: {}", command, name);
}
