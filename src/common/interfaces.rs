use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{result::TaskResult, task::Task};

pub trait GARunner: Send + Sync + 'static {
    fn run(&self, task: Task, worker_id: Uuid) -> TaskResult;
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DummyGARunner;

impl GARunner for DummyGARunner {
    fn run(&self, task: Task, worker_id: Uuid) -> TaskResult {
        todo!()
    }
}
