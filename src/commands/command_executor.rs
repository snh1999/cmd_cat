// use std::{
//     env,
//     path::Path,
//     process::{exit, Child, Command, Stdio},
// };

// pub struct CommandExecutor {}

// impl CommandExecutor {
//     pub fn new() -> Self {
//         CommandExecutor {}
//     }

//     pub fn execute_command(&self, command: &str) {
//         print!(">>");
//         let mut parts = command.split_whitespace();
//         if let Some(command_name) = parts.next() {
//             let mut process = Command::new(command_name);
//             process.args(parts);

//             let output = match process.output() {
//                 Ok(output) => output,
//                 Err(error) => {
//                     eprintln!("Failed to execute command '{}': {}", command, error);
//                     exit(1);
//                 }
//             };

//             if !output.stdout.is_empty() {
//                 let stdout = String::from_utf8(output.stdout).unwrap();
//                 println!("{}", stdout);
//             }

//             if !output.stderr.is_empty() {
//                 let stderr = String::from_utf8_lossy(&output.stderr);
//                 eprintln!("{}", stderr);
//             }
//         } else {
//             println!("Invalid command");
//         }
//     }
// }
// incl piping
// fn execute_command(input: &str) {
//     // must be peekable so we know when we are on the last command
//     let mut commands = input.trim().split(" | ").peekable();
//     let mut previous_command = None;

//     while let Some(command) = commands.next() {
//         let mut parts = command.trim().split_whitespace();
//         let command = parts.next().unwrap();
//         let args = parts;

//         match command {
//             "cd" => {
//                 let new_dir = args.peekable().peek().map_or("/", |x| *x);
//                 let root = Path::new(new_dir);
//                 if let Err(e) = env::set_current_dir(&root) {
//                     eprintln!("{}", e);
//                 }

//                 previous_command = None;
//             }
//             "exit" => return,
//             command => {
//                 let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
//                     Stdio::from(output.stdout.unwrap())
//                 });

//                 let stdout = if commands.peek().is_some() {
//                     // there is another command piped behind this one
//                     // prepare to send output to the next command
//                     Stdio::piped()
//                 } else {
//                     // there are no more commands piped behind this one
//                     // send output to shell stdout
//                     Stdio::inherit()
//                 };

//                 let output = Command::new(command)
//                     .args(args)
//                     .stdin(stdin)
//                     .stdout(stdout)
//                     .spawn();

//                 match output {
//                     Ok(output) => {
//                         previous_command = Some(output);
//                     }
//                     Err(e) => {
//                         previous_command = None;
//                         eprintln!("{}", e);
//                     }
//                 };
//             }
//         }
//     }

//     if let Some(mut final_command) = previous_command {
//         // block until the final command has finished
//         final_command.wait();
//     }
// }

use std::{env, path::Path, process::Command};

pub fn execute_command(input: &str) {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    match command {
        "cd" => {
            let new_dir = args.peekable().peek().map_or("/", |x| *x);
            let root = Path::new(new_dir);
            if let Err(e) = env::set_current_dir(&root) {
                eprintln!("{}", e);
            }
        }
        "exit" => return,
        command => {
            let child = Command::new(command).args(args).spawn();

            // gracefully handle malformed user input
            match child {
                Ok(mut child) => {
                    let _ = child.wait();
                }
                Err(e) => eprintln!("{}", e),
            };
        }
    }
}
