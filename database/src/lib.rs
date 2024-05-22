#![allow(unused)]

mod db_tests;
mod models;

use crate::models::DBState;
use anyhow::Result;


trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        todo!() // read the content's of self.file_path and deserialize it using serde
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        todo!() // serialize db_state to json and store it in self.file_path
    }
}
