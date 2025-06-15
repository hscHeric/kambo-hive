use serde::{Deserialize, Serialize};

use super::{result::TaskResult, task::Task};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    RequestTask {
        worker_id: String,
    },
    ReportResult {
        worker_id: String,
        result: TaskResult,
    },
    Heartbeat {
        worker_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    AssignTask {
        task: Task,
    },
    NoTaskAvailable,
    Ack,
    Command {
        command_type: String,
        payload: String,
    },
}
