use std::collections::{HashMap, VecDeque};

use uuid::Uuid;

use crate::common::Task;

pub enum TaskStatus {
    Pending,
    Assigned,
    Completed,
    Failed,
}

pub struct TaskManager {
    pending_tasks: VecDeque<Task>,
    assigned_tasks: HashMap<Uuid, (Task, Uuid)>, // (TaskId, (task, id do worker))
    all_tasks_status: HashMap<Uuid, TaskStatus>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            pending_tasks: VecDeque::new(),
            assigned_tasks: HashMap::new(),
            all_tasks_status: HashMap::new(),
        }
    }
}
