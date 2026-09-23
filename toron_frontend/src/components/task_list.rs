use crate::models::{SyncStatus, Task};
use dioxus::prelude::*;

#[component]
pub fn TaskList(tasks: Vec<Task>) -> Element {
    rsx! {
        if tasks.is_empty() {
            p { "Aucune tâche pour le moment." }
        } else {
            ul {
                for task in tasks {
                    li { key: "{task.id}",
                        input {
                            r#type: "checkbox",
                            checked: task.completed,
                            disabled: true,
                        }
                        span { " {task.title}" }
                        small { " — {sync_status_label(&task.sync_status)}" }
                    }
                }
            }
        }
    }
}

fn sync_status_label(status: &SyncStatus) -> &'static str {
    match status {
        SyncStatus::Pending => "en attente de synchronisation",
        SyncStatus::Synced => "synchronisée",
        SyncStatus::Failed => "échec de synchronisation",
    }
}
