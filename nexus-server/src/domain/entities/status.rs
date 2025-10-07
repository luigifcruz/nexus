#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Unknown,
    Standby,
    Starting,
    Running,
    Stopping,
    Stopped,
    Errored,
    Maintenance,
    Disconnected,
}

impl Default for Status {
    fn default() -> Self {
        Status::Unknown
    }
}

impl Status {
    pub fn is_active(&self) -> bool {
        matches!(self, Status::Running | Status::Starting)
    }

    pub fn is_inactive(&self) -> bool {
        matches!(self, Status::Stopped | Status::Stopping)
    }

    pub fn is_disconnected(&self) -> bool {
        matches!(self, Status::Disconnected)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Status::Errored)
    }
}

impl From<nexus_common::proto::enums::Status> for Status {
    fn from(proto_status: nexus_common::proto::enums::Status) -> Self {
        match proto_status {
            nexus_common::proto::enums::Status::Unknown => Status::Unknown,
            nexus_common::proto::enums::Status::Standby => Status::Standby,
            nexus_common::proto::enums::Status::Starting => Status::Starting,
            nexus_common::proto::enums::Status::Running => Status::Running,
            nexus_common::proto::enums::Status::Stopping => Status::Stopping,
            nexus_common::proto::enums::Status::Stopped => Status::Stopped,
            nexus_common::proto::enums::Status::Errored => Status::Errored,
            nexus_common::proto::enums::Status::Maintenance => Status::Maintenance,
            nexus_common::proto::enums::Status::Disconnected => Status::Disconnected,
        }
    }
}

impl From<Status> for nexus_common::proto::enums::Status {
    fn from(status: Status) -> Self {
        match status {
            Status::Unknown => nexus_common::proto::enums::Status::Unknown,
            Status::Standby => nexus_common::proto::enums::Status::Standby,
            Status::Starting => nexus_common::proto::enums::Status::Starting,
            Status::Running => nexus_common::proto::enums::Status::Running,
            Status::Stopping => nexus_common::proto::enums::Status::Stopping,
            Status::Stopped => nexus_common::proto::enums::Status::Stopped,
            Status::Errored => nexus_common::proto::enums::Status::Errored,
            Status::Maintenance => nexus_common::proto::enums::Status::Maintenance,
            Status::Disconnected => nexus_common::proto::enums::Status::Disconnected,
        }
    }
}
