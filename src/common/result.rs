use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub worker_id: String,
    pub fitness: u64,
    pub solution_data: Vec<u8>,
    pub processing_time_ms: u64,
}
