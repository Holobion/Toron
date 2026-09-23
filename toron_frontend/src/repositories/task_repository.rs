use crate::models::Task;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum TaskRepositoryError {
    InvalidTitle,
    NotFound(String),
}

#[allow(dead_code)]
pub trait TaskRepository {
    fn list(&self) -> Result<Vec<Task>, TaskRepositoryError>;
    fn insert(&mut self, task: Task) -> Result<Task, TaskRepositoryError>;
    fn update(&mut self, task: Task) -> Result<Task, TaskRepositoryError>;
    fn delete(&mut self, id: &str) -> Result<(), TaskRepositoryError>;
}
