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
        log::info!(
            "Worker {worker_id} está executando a task {} (Grafo: {}, Execução: {})",
            task.graph_id,
            task.graph_id,
            task.run_number
        );

        std::thread::sleep(std::time::Duration::from_secs(1));

        TaskResult {
            task_id: task.id,
            worker_id,
            fitness: f64::from(task.run_number) * 100.0 / 30.0 + rand::random::<f64>() * 10.0, // Calculo
            // totalmente sem sentido para gerar um fitness aleatorio
            solution_data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            interations_run: 1000,
            processing_time_ms: 1000,
            graph_id: task.graph_id.clone(),
        }
    }
}
