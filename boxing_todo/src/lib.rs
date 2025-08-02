mod err;

use std::error::Error;

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
    pub fn get_todo(file_name: &str) -> Result<Self, Box<dyn Error>> {
        
        let content = std::fs::read_to_string(file_name)
            .map_err(|e| Box::new(err::ReadErr { child_err: Box::new(e) }))?;

        
        if content.trim().is_empty() {
            return Err(Box::new(err::ParseErr::Empty));
        }

        
        let parsed = json::parse(&content)
            .map_err(|e| Box::new(err::ParseErr::Malformed(Box::new(e))))?;

        let todo_list = (|| {
            let title = parsed["title"].as_str().ok_or("title field is missing or not a string")?.to_string();
            let mut tasks = Vec::new();
            let tasks_json = parsed["tasks"].is_array()?;

            for task_json in tasks_json {
                let id = task_json["id"].as_u32().ok_or("task `id` is missing or invalid")?;
                let description = task_json["description"].as_str().ok_or("task `description` is missing or invalid")?.to_string();
                let level = task_json["level"].as_u32().ok_or("task `level` is missing or invalid")?;
                tasks.push(Task { id, description, level });
            }
            Ok::<_, &str>(TodoList { title, tasks })
        })().map_err(|e: &str| Box::new(err::ParseErr::Malformed(e.into())))?;

        
        if todo_list.tasks.is_empty() {
            return Err(Box::new(err::ParseErr::Empty));
        }

        Ok(todo_list)
    }
}


















































