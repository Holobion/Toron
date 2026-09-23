use crate::{
    models::{NewTask, SyncStatus, Task},
    repositories::{TaskRepository, TaskRepositoryError},
};

pub struct TaskService<R> {
    repository: R,
}

#[allow(dead_code)]
impl<R> TaskService<R>
where
    R: TaskRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn list(&self) -> Result<Vec<Task>, TaskRepositoryError> {
        self.repository.list()
    }

    pub fn create(&mut self, input: NewTask) -> Result<Task, TaskRepositoryError> {
        self.repository.insert(Task {
            id: String::new(),
            title: input.title,
            completed: false,
            created_at: 0,
            updated_at: 0,
            sync_status: SyncStatus::Pending,
        })
    }

    pub fn set_completed(
        &mut self,
        id: &str,
        completed: bool,
    ) -> Result<Task, TaskRepositoryError> {
        let mut task = self
            .repository
            .list()?
            .into_iter()
            .find(|task| task.id == id)
            .ok_or_else(|| TaskRepositoryError::NotFound(id.to_string()))?;
        task.completed = completed;
        self.repository.update(task)
    }

    pub fn rename(
        &mut self,
        id: &str,
        title: impl Into<String>,
    ) -> Result<Task, TaskRepositoryError> {
        let mut task = self
            .repository
            .list()?
            .into_iter()
            .find(|task| task.id == id)
            .ok_or_else(|| TaskRepositoryError::NotFound(id.to_string()))?;
        task.title = title.into();
        self.repository.update(task)
    }

    pub fn delete(&mut self, id: &str) -> Result<(), TaskRepositoryError> {
        self.repository.delete(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::MockTaskRepository;

    #[test]
    fn creates_tasks_locally_and_marks_them_pending() {
        let mut service = TaskService::new(MockTaskRepository::new());

        let task = service
            .create(NewTask::new("Acheter du pain"))
            .expect("task should be created");

        assert_eq!(task.title, "Acheter du pain");
        assert_eq!(task.sync_status, SyncStatus::Pending);
        assert_eq!(service.list().expect("tasks should be listed").len(), 1);
    }

    #[test]
    fn updates_and_deletes_tasks_through_the_repository() {
        let mut service = TaskService::new(MockTaskRepository::new());
        let task = service
            .create(NewTask::new("Lire"))
            .expect("task should be created");

        let updated = service
            .set_completed(&task.id, true)
            .expect("task should be updated");
        assert!(updated.completed);
        assert_eq!(updated.sync_status, SyncStatus::Pending);

        service.delete(&task.id).expect("task should be deleted");
        assert!(service.list().expect("tasks should be listed").is_empty());
    }

    #[test]
    fn renames_tasks_through_the_service() {
        let mut service = TaskService::new(MockTaskRepository::new());
        let task = service
            .create(NewTask::new("Lire"))
            .expect("task should be created");

        let renamed = service
            .rename(&task.id, "Lire un chapitre")
            .expect("task should be renamed");

        assert_eq!(renamed.title, "Lire un chapitre");
        assert!(renamed.updated_at > renamed.created_at);
    }

    #[test]
    fn rejects_empty_titles() {
        let mut service = TaskService::new(MockTaskRepository::new());

        assert_eq!(
            service.create(NewTask::new("  ")),
            Err(TaskRepositoryError::InvalidTitle)
        );
    }
}
