use crate::models::{SyncStatus, Task};

use super::{TaskRepository, TaskRepositoryError};

#[derive(Clone, Debug, Default)]
pub struct MockTaskRepository {
    tasks: Vec<Task>,
    next_id: u64,
    clock: u64,
}

impl MockTaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
            clock: 1,
        }
    }

    pub fn with_seed_data() -> Self {
        let mut repository = Self::new();
        repository
            .insert(Task {
                id: "mock-1".to_string(),
                title: "Préparer la première tâche".to_string(),
                completed: false,
                created_at: 1,
                updated_at: 1,
                sync_status: SyncStatus::Synced,
            })
            .expect("seed data should be valid");
        repository.tasks[0].sync_status = SyncStatus::Synced;
        repository
    }

    fn next_timestamp(&mut self) -> u64 {
        self.clock += 1;
        self.clock
    }
}

impl TaskRepository for MockTaskRepository {
    fn list(&self) -> Result<Vec<Task>, TaskRepositoryError> {
        Ok(self.tasks.clone())
    }

    fn insert(&mut self, mut task: Task) -> Result<Task, TaskRepositoryError> {
        if task.title.trim().is_empty() {
            return Err(TaskRepositoryError::InvalidTitle);
        }

        if task.id.is_empty() {
            task.id = format!("mock-{}", self.next_id);
            self.next_id += 1;
        }

        if task.created_at == 0 {
            task.created_at = self.next_timestamp();
            task.updated_at = task.created_at;
        }
        task.sync_status = SyncStatus::Pending;
        self.tasks.push(task.clone());
        Ok(task)
    }

    fn update(&mut self, mut task: Task) -> Result<Task, TaskRepositoryError> {
        if task.title.trim().is_empty() {
            return Err(TaskRepositoryError::InvalidTitle);
        }

        let position = self
            .tasks
            .iter()
            .position(|stored| stored.id == task.id)
            .ok_or_else(|| TaskRepositoryError::NotFound(task.id.clone()))?;

        task.updated_at = self.next_timestamp();
        task.sync_status = SyncStatus::Pending;
        self.tasks[position] = task.clone();
        Ok(task)
    }

    fn delete(&mut self, id: &str) -> Result<(), TaskRepositoryError> {
        let position = self
            .tasks
            .iter()
            .position(|task| task.id == id)
            .ok_or_else(|| TaskRepositoryError::NotFound(id.to_string()))?;

        self.tasks.remove(position);
        Ok(())
    }
}
