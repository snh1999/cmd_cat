// use std::fs;
// // use std::path::Path;

// fn main() {
//     let filepath = "./yay.md";
//     let file_content = read_file(filepath);
//     // let dirpath = "tldr-analyzer/pages/android";
//     // let filenames= read_folder(dirpath);
//     // for filename in filenames {
//     //     println!("{}", filename);
//     // }
//     let results = extract_key_value_from_file(&file_content);
//     // for (key, value) in results {
//     //     println!("Key: {}", key);
//     //     println!("Value: {}", value);
//     //     println!();
//     // }
// }

// fn read_file(filepath: &str) -> String {
//     // let mut file = File::open(filepath).expect("Failed to open file");
//     // let mut contents = String::new();
//     // file.read_to_string(&mut contents).expect("Failed to read file");
//     // println!("{}", contents);

//     let contents =
//         fs::read_to_string(filepath).expect(&format!("Failed to read file: {}", filepath));
//     contents
// }
// // value is after -
// // next    ` to other ` is the key
// // store the first word somewhere

// fn read_folder(dirpath: &str) -> Vec<String> {
//     // let dir = fs::read_dir(dirpath).expect(&format!("Failed to read file: {}", dirpath));
//     // for file in dir {
//     //     println!("{file:?}");//, file = file?.path())
//     // }
//     let mut filenames = Vec::new();

//     if let Ok(entries) = fs::read_dir(dirpath) {
//         for entry in entries {
//             if let Ok(entry) = entry {
//                 let path = entry.path();
//                 if path.is_file() {
//                     if let Some(filename) = path.file_name() {
//                         if let Some(filename_str) = filename.to_str() {
//                             filenames.push(filename_str.to_owned());
//                         }
//                     }
//                 } else if path.is_dir() {
//                     let sub_dir_path = path.to_str().unwrap();
//                     filenames.extend(read_folder(sub_dir_path));
//                 }
//             }
//         }
//     }

//     filenames
// }

// fn extract_key_value_from_file(input: &str) -> Vec<(String, String)> {
//     let mut results = Vec::new();
//     let lines = input.lines();

//     let mut current_key = String::new();
//     let mut current_value = String::new();

//     for line in lines {
//         let trimmed_line = line.trim();

//         if trimmed_line.starts_with('-') {
//             if !current_key.is_empty() && !current_value.is_empty() {
//                 results.push((current_key.clone(), current_value.clone()));
//                 current_key.clear();
//                 current_value.clear();
//             }
//             current_value = trimmed_line.trim_start_matches('-').trim().to_owned();
//         } else if trimmed_line.starts_with('`') && trimmed_line.ends_with('`') {
//             current_key = trimmed_line.trim_matches('`').trim().to_owned();
//         }
//     }

//     if !current_key.is_empty() && !current_value.is_empty() {
//         results.push((current_key, current_value));
//     }

//     results
// }

use crate::command::Command;
use crate::database::SqliteDatabase;
use std::fs;

pub fn insert_commands_from_file(filepath: &str, db: &SqliteDatabase) {
    let file_content = read_file(filepath);
    let results = extract_key_value_from_file(&file_content);

    for (command_name, description) in results {
        let command = Command::new(&command_name, &description);
        db.insert(&command)
            .expect("Failed to insert command into the database");
    }
}

pub fn clean_update_database(db: &SqliteDatabase, folder_path: &str) {
    db.clear();
    update_database(db, folder_path)
}

pub fn update_database(db: &SqliteDatabase, folder_path: &str) {
    let filenames = read_folder(folder_path);
    for filename in filenames {
        let file_path = format!("{}/{}", folder_path, filename);
        insert_commands_from_file(&file_path, db);
    }
}

pub fn read_file(filepath: &str) -> String {
    let contents =
        fs::read_to_string(filepath).expect(&format!("Failed to read file: {}", filepath));
    contents
}

/// returns all the filename from folders
pub fn read_folder(dirpath: &str) -> Vec<String> {
    let mut filenames = Vec::new();

    if let Ok(entries) = fs::read_dir(dirpath) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name() {
                        if let Some(filename_str) = filename.to_str() {
                            filenames.push(filename_str.to_owned());
                        }
                    }
                } else if path.is_dir() {
                    let sub_dir_path = path.to_str().unwrap();
                    filenames.extend(read_folder(sub_dir_path));
                }
            }
        }
    }

    filenames
}

pub fn extract_key_value_from_file(input: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let lines = input.lines();

    let mut current_key = String::new();
    let mut current_value = String::new();

    for line in lines {
        let trimmed_line = line.trim();

        if trimmed_line.starts_with('-') {
            if !current_key.is_empty() && !current_value.is_empty() {
                results.push((current_key.clone(), current_value.clone()));
                current_key.clear();
                current_value.clear();
            }
            current_value = trimmed_line.trim_start_matches('-').trim().to_owned();
        } else if trimmed_line.starts_with('`') && trimmed_line.ends_with('`') {
            current_key = trimmed_line.trim_matches('`').trim().to_owned();
        }
    }

    if !current_key.is_empty() && !current_value.is_empty() {
        results.push((current_key, current_value));
    }

    results
}
