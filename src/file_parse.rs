use crate::commands::Command;
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

pub fn _clean_update_database(db: &SqliteDatabase, folder_path: &str) {
    db.clear().unwrap();
    _update_database(db, folder_path)
}

pub fn _update_database(db: &SqliteDatabase, folder_path: &str) {
    let filenames = _read_folder(folder_path);
    for filename in filenames {
        println!("Processing: {filename}");
        insert_commands_from_file(&filename, db);
    }
}

pub fn read_file(filepath: &str) -> String {
    let contents =
        fs::read_to_string(filepath).expect(&format!("Failed to read file: {}", filepath));
    contents
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

/// returns all the filename from folders
fn _read_folder(dirpath: &str) -> Vec<String> {
    let mut filepaths = Vec::new();

    if let Ok(entries) = fs::read_dir(dirpath) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filepath_str) = path.to_str() {
                        filepaths.push(filepath_str.to_owned());
                    }
                } else if path.is_dir() {
                    let sub_dir_path = path.to_str().unwrap();
                    let mut sub_dir_filepaths = _read_folder(sub_dir_path);
                    filepaths.append(&mut sub_dir_filepaths);
                }
            }
        }
    }

    filepaths
}
