use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBUser {
    pub name: String,
    pub age: u8,
    pub id: Thing,
}
