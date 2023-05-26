use crate::commands::Command;
use rusqlite::{params, Connection, Result};

pub struct SqliteDatabase {
    connection: Connection,
}

impl SqliteDatabase {
    pub fn new() -> Result<Self> {
        let connection = Connection::open("commands.db")?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS commands (
                id INTEGER PRIMARY KEY,
                command_name TEXT NOT NULL,
                description TEXT NOT NULL
            )",
            [],
        )?;
        Ok(SqliteDatabase { connection })
    }

    pub fn insert(&self, command: &Command) -> Result<()> {
        self.connection.execute(
            "INSERT INTO commands (command_name, description) VALUES (?1, ?2)",
            [command.name.as_str(), command.description.as_str()],
        )?;
        Ok(())
    }

    pub fn get_command_description(&self, command_name: &str) -> Result<Option<String>> {
        let mut stmt = self
            .connection
            .prepare("SELECT description FROM commands WHERE command_name = ?1")?;
        let result = stmt.query_row([command_name], |row| row.get(0));
        match result {
            Ok(description) => Ok(Some(description)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn find_matching_commands(&self, prefix: &str) -> Result<Vec<(String, String)>> {
        let mut stmt = self
            .connection
            .prepare("SELECT command_name, description FROM commands WHERE command_name LIKE ?1")?;
        let mut rows = stmt.query([format!("{}%", prefix)])?;

        let mut matching_commands = Vec::new();
        while let Some(row) = rows.next()? {
            let command_name: String = row.get(0)?;
            let description: String = row.get(1)?;
            matching_commands.push((command_name, description));
        }
        Ok(matching_commands)
    }

    pub fn _view_all_commands(&self) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare("SELECT command_name, description FROM commands")?;
        let mut rows = stmt.query([])?;

        println!("All commands:");
        while let Some(row) = rows.next()? {
            let command_name: String = row.get(0)?;
            let description: String = row.get(1)?;
            println!("Command: {}", command_name);
            println!("Description: {}", description);
            println!("-------------------------");
        }
        Ok(())
    }

    pub fn search_commands(&self, input: &str) -> Result<Vec<(String, String)>> {
        let search_words: Vec<&str> = input.split_whitespace().collect();

        let mut matching_commands = Vec::new();
        let mut command_ids = Vec::new();

        // Step 1: Search using the first word and store the results
        let first_word = search_words.get(0).cloned().unwrap_or("");
        let mut stmt = self.connection.prepare("SELECT id, command_name, description FROM commands WHERE command_name LIKE ?1 OR description LIKE ?1")?;
        let mut rows = stmt.query(params![format!("%{}%", first_word)])?;
        while let Some(row) = rows.next()? {
            let command_id: i32 = row.get(0)?;
            let command_name: String = row.get(1)?;
            let description: String = row.get(2)?;
            matching_commands.push((command_name.clone(), description.clone()));
            command_ids.push(command_id);
        }

        // Step 2: Filter the results for each subsequent word
        for word in search_words.iter().skip(1) {
            let mut new_matching_commands = Vec::new();
            let mut new_command_ids = Vec::new();

            for (command_id, (command_name, description)) in
                command_ids.iter().zip(matching_commands.iter())
            {
                // Check if the word is present in the command name or description
                if command_name.contains(word) || description.contains(word) {
                    new_matching_commands.push((command_name.clone(), description.clone()));
                    new_command_ids.push(*command_id);
                }
            }

            matching_commands = new_matching_commands;
            command_ids = new_command_ids;
        }

        Ok(matching_commands)
    }

    pub fn clear(&self) -> Result<()> {
        self.connection.execute("DELETE FROM commands", [])?;
        Ok(())
    }
}
