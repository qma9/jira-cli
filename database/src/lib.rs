#![allow(unused)]

mod db_tests;
mod models;

use anyhow::{Error, Ok, Result};
use models::DBState;
use serde::{Deserialize, Serialize};
use std::fs;

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

#[derive(Serialize, Deserialize, Debug)]
struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        // Read the contents of self.file_path and deserialize it using serde
        let contents = fs::read_to_string(&self.file_path)?;
        let deserialize: DBState = serde_json::from_str(&contents)?;
        Ok(deserialize)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        // Serialize db_state to json and store it in self.file_path
        let serialized = serde_json::to_string(db_state)?;
        let contents = fs::write(&self.file_path, serialized);
        Ok(())
    }
}
