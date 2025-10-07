use chrono::{DateTime, Utc};

use super::Status;

#[derive(Debug, Clone)]
pub struct Instance {
    pub id: String,
    pub replicant_id: String,
    pub tags: Vec<String>,
    pub status: Status,
    pub docker_image: String,
    pub docker_args: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
}

impl Instance {
    pub fn new(
        id: String,
        replicant_id: String,
        docker_image: String,
        docker_args: Vec<String>,
        tags: Vec<String>,
    ) -> Self {
        Self {
            id,
            replicant_id,
            tags,
            status: Status::Unknown,
            docker_image,
            docker_args,
            created_at: Utc::now(),
            started_at: None,
        }
    }

    pub fn update_status(&mut self, status: Status) {
        self.status = status;
        if status == Status::Running && self.started_at.is_none() {
            self.started_at = Some(Utc::now());
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.status, Status::Running)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self.status, Status::Stopped)
    }
}
