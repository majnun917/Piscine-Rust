mod err;

use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let content = std::fs::read_to_string(file_name)
            .map_err(|e| err::ReadErr { child_err: Box::new(e) })?;
        
        let todo_list: TodoList = serde_json::from_str(&content)
            .map_err(|e| {
                if content.trim().is_empty() {
                    err::ParseErr::Empty
                } else {
                    err::ParseErr::Malformed(Box::new(e))
                }
            })?;
        
        if todo_list.tasks.is_empty() {
            return Err(Box::new(err::ParseErr::Empty));
        }
        
        Ok(todo_list)
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs::File;
//     use std::io::Write;

//     #[test]
//     fn test_get_todo_ok() {
//         let content = r#"{"title":"My List","tasks":[{"id":0,"description":"task 1","level":1}]}"#;
//         File::create("todo.json").unwrap().write_all(content.as_bytes()).unwrap();
//         let todo_list = TodoList::get_todo("todo.json").unwrap();
//         assert_eq!(todo_list.title, "My List");
//         assert_eq!(todo_list.tasks.len(), 1);
//         std::fs::remove_file("todo.json").unwrap();
//     }

//     #[test]
//     fn test_get_todo_file_not_found() {
//         let result = TodoList::get_todo("non_existent_file.json");
//         assert!(result.is_err());
//         let err = result.unwrap_err();
//         // On vérifie que le message d'erreur est bien celui de ReadErr
//         assert_eq!(err.to_string(), "Failed to read todo file");
//         // On vérifie que le type sous-jacent est bien ReadErr
//         assert!(err.is::<err::ReadErr>());
//     }

//     #[test]
//     fn test_get_todo_malformed() {
//         File::create("malformed.json").unwrap().write_all(b"{'bad json'").unwrap();
//         let result = TodoList::get_todo("malformed.json");
//         let err = result.unwrap_err();
//         assert_eq!(err.to_string(), "Failed to parse todo file");
//         assert!(err.downcast_ref::<err::ParseErr>().is_some());
//         std::fs::remove_file("malformed.json").unwrap();
//     }

//     #[test]
//     fn test_get_todo_empty() {
//         let content = r#"{"title":"Empty List","tasks":[]}"#;
//         File::create("empty.json").unwrap().write_all(content.as_bytes()).unwrap();
//         let result = TodoList::get_todo("empty.json");
//         let err = result.unwrap_err();
//         assert_eq!(err.to_string(), "Failed to parse todo file");
//         assert!(matches!(err.downcast_ref::<err::ParseErr>().unwrap(), err::ParseErr::Empty));
//         std::fs::remove_file("empty.json").unwrap();
//     }
// }
