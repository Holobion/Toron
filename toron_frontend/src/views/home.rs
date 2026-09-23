use crate::{components::TaskList, repositories::MockTaskRepository, services::TaskService};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let task_service = use_signal(|| TaskService::new(MockTaskRepository::with_seed_data()));
    let tasks = task_service.read().list().unwrap_or_default();

    rsx! {
        section { id: "tasks",
            h2 { "Mes tâches" }
            TaskList { tasks }
        }
    }
}
