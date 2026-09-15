mod err;

use std::{fs, error::Error};

pub use crate::err::{ParseErr, ReadErr};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let contents = fs::read_to_string(path).map_err(|e| ReadErr { child_err: Box::new(e) })?;
        let parsed = json::parse(&contents).map_err(|err| ParseErr::Malformed(Box::new(err)))?;
        if parsed["tasks"].is_empty() {
            return Err(Box::new(ParseErr::Empty))
        }
        let tasks: Vec<Task> = parsed["tasks"]
            .members()
            .map(|task| Task {
                id: task["id"].as_u32().unwrap(),
                description: task["description"].as_str().unwrap().to_string(),
                level: task["level"].as_u32().unwrap(),
            })
            .collect();

        Ok(TodoList {
            title: parsed["title"].as_str().unwrap().to_string(),
            tasks,
        })
    }
}
