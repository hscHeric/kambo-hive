mod genetic_algorithm;
mod graph;

use genetic_algorithm::RomanDominationGA;
use graph::Graph;
use kambo_hive::common::{GARunner, Task, TaskResult};
use kambo_hive::utils::init_logger;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{env, path::Path, sync::Arc, time::Instant};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct GAConfig {
    pub max_stagnant: usize,
    pub generations: usize,
    pub tournament_size: usize,
    pub crossover_probability: f32,
    pub pop_size: Option<usize>,
}

struct RomanDominationGARunner {
    graphs_path: String,
}

impl GARunner for RomanDominationGARunner {
    fn run(&self, task: Task, worker_id: Uuid) -> TaskResult {
        info!(
            "Worker {} processando a task {} para o grafo '{}'",
            worker_id, task.id, task.graph_id
        );

        let start_time = Instant::now();

        let ga_config: GAConfig = serde_json::from_str(&task.ag_config)
            .expect("Falha ao deserializar a configuração do AG");

        let graph_file_path = Path::new(&self.graphs_path)
            .join(&task.graph_id)
            .to_str()
            .unwrap()
            .to_string();

        let graph =
            Graph::from_file(graph_file_path).expect("Falha ao carregar o arquivo do grafo");

        let mut rdga = RomanDominationGA::new(graph, ga_config.pop_size);
        let solution = rdga.run(
            ga_config.generations,
            ga_config.max_stagnant,
            ga_config.tournament_size,
            ga_config.crossover_probability,
        );

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        let fitness_value = solution.fitness.map_or(-1.0, |f| f as f64);
        info!(
            "Task {} finalizada para o grafo '{}' com fitness de {}",
            task.id, task.graph_id, fitness_value
        );

        TaskResult {
            task_id: task.id,
            graph_id: task.graph_id.clone(),
            worker_id,
            fitness: fitness_value,
            solution_data: Vec::new(), // Opcional: pode serializar `solution.labels` aqui
            interations_run: ga_config.generations as u32,
            processing_time_ms,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Uso: {} <host_addr:port> <graphs_path>", args[0]);
        std::process::exit(1);
    }

    let host_addr = &args[1];
    let graphs_path = &args[2];
    let worker_id = Uuid::new_v4();

    info!("Iniciando worker {}...", worker_id);

    let ga_runner = Arc::new(RomanDominationGARunner {
        graphs_path: graphs_path.clone(),
    });

    if let Err(e) = kambo_hive::worker::client::start_worker(host_addr, worker_id, ga_runner).await
    {
        error!("Erro fatal no worker: {}", e);
    }

    Ok(())
}
