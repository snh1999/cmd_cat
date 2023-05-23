use crate::command::Command;
use rusqlite::{Connection, Result};

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
    pub fn clear(&self) -> Result<()> {
        self.connection.execute("DELETE FROM commands", [])?;
        Ok(())
    }
}
