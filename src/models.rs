use crate::schema::todos;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub public_id: &'a str,
    pub description: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id: i32,
    pub public_id: String,
    pub title: String,
    pub description: String,
    pub completed: i32,
}
