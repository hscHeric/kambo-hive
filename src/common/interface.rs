use uuid::Uuid;

use super::task::Task;

pub trait GARunner: Send + Sync + 'static {
    fn run(&self, task: Task, worker_id: Uuid) -> TaskResult;
}
