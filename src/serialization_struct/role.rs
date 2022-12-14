use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub description: String
}