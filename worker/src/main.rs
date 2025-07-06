use std::{env::args, path::Path, sync::Arc, time::Instant};

use kambo_hive::{
    common::{GARunner, Task, TaskResult},
    utils::{discover_host, init_logger},
    worker::client::start_worker,
};
use kambo_hive_worker::graph::Graph;
use log::{error, info};
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct GAConfig {
    pub max_stagnant: usize,
    pub generations: usize,
    pub tournament_size: usize,
    pub crossover_probability: f32,
    pub pop_size: Option<usize>,
}

pub struct HeuristicRunner {
    graphs_path: String,
}

impl GARunner for HeuristicRunner {
    fn run(&self, task: Task, worker_id: Uuid) -> TaskResult {
        info!(
            "Worker {} processando a task {} para o grafo '{}'",
            worker_id, task.id, task.graph_id
        );

        // Aqui eu coloquei só a leitura e não estou usando, mas poderia ser usado para configurar
        // o algoritmo, uma opbservação é que cada task poderia usar uma confgiuração diferente.
        let _ga_config: GAConfig = serde_json::from_str(&task.ag_config)
            .expect("Falha ao deserializar a GAConfig da task. A configuração é necessária.");

        let start_time = Instant::now();
        let graph_file_path = Path::new(&self.graphs_path)
            .join(format!("{}", &task.graph_id))
            .to_str()
            .unwrap()
            .to_string();

        info!("Carregando grafo de: {graph_file_path}");
        let graph =
            Graph::from_file(&graph_file_path).expect("Falha ao carregar o arquivo do grafo");

        let heuristic_choice = rng().random_range(1..=4);
        let solution_data = match heuristic_choice {
            1 => graph.h1(),
            2 => graph.h2(),
            3 => graph.h3(),
            4 => graph.h4(),
            _ => unreachable!(), // gen_range(1..=4) garante que isso nunca aconteça
        };

        let fitness = solution_data.iter().map(|&value| f64::from(value)).sum();
        let processing_time_ms = start_time.elapsed().as_millis() as u64;
        info!(
            "Task {} finalizada para o grafo '{}' com fitness de {} (usando H{})",
            task.id, task.graph_id, fitness, heuristic_choice
        );

        TaskResult {
            task_id: task.id,
            graph_id: task.graph_id,
            worker_id,
            fitness,
            solution_data: Vec::new(),
            interations_run: graph.get_num_vertices() as u32,
            processing_time_ms,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        eprintln!("Uso: {} <host_addr:port | --auto> <graphs_path>", args[0]);
        std::process::exit(1);
    }

    let host_addr = if args[1] == "--auto" {
        info!("Iniciando descoberta automática de host...");
        match discover_host() {
            Ok(addr) => {
                info!("Host encontrado com sucesso em: {addr}");
                addr
            }
            Err(e) => {
                error!("Falha na descoberta automática: {e}");
                std::process::exit(1);
            }
        }
    } else {
        args[1].clone()
    };

    let graphs_path = &args[2];
    let worker_id = Uuid::new_v4();

    info!("Iniciando worker {worker_id}...");
    info!("Conectando ao host: {host_addr}");
    info!("Usando grafos de: {graphs_path}");

    let ga_runner = Arc::new(HeuristicRunner {
        graphs_path: graphs_path.clone(),
    });

    if let Err(e) = start_worker(&host_addr, worker_id, ga_runner).await {
        error!("Erro fatal no worker: {e}");
    }

    Ok(())
}
