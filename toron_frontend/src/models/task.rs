#[derive(Clone, Debug, PartialEq)]
pub enum SyncStatus {
    Pending,
    Synced,
    #[allow(dead_code)]
    Failed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub sync_status: SyncStatus,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct NewTask {
    pub title: String,
}

#[allow(dead_code)]
impl NewTask {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}
