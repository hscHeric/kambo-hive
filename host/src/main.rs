use kambo_hive::host::{
    periodic_saver,
    result_aggregator::ResultAggregator,
    server::start_server,
    task_manager::{DistributionStrategy, TaskManager},
};
use kambo_hive::utils::{init_logger, listen_for_workers};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::{env, fs, process, sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct GAConfig {
    pub trials: u32,
    pub max_stagnant: usize,
    pub generations: usize,
    pub tournament_size: usize,
    pub crossover_probability: f32,
    pub pop_size: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!(
            "Uso: {} <bind_addr:port> <graphs_path> <report_path> <strategy> [save_path] [save_interval_secs]",
            args[0]
        );
        eprintln!("Estratégias disponíveis: fifo, lifo, random");
        eprintln!(
            "Exemplo: {} 0.0.0.0:12345 ./graphs final_report.json fifo results.json 60",
            args[0]
        );
        process::exit(1);
    }

    let bind_addr = &args[1];
    let graphs_path = &args[2];
    let report_path = &args[3];
    let strategy_str = &args[4];
    let save_path = args.get(5);
    let save_interval = args.get(6).and_then(|s| s.parse().ok());

    let distribution_strategy = match strategy_str.to_lowercase().as_str() {
        "fifo" => DistributionStrategy::Fifo,
        "lifo" => DistributionStrategy::Lifo,
        "random" => DistributionStrategy::Random,
        _ => {
            error!(
                "Estratégia de distribuição inválida: '{}'. Use 'fifo', 'lifo', ou 'random'.",
                strategy_str
            );
            process::exit(1);
        }
    };
    info!(
        "Usando a estratégia de distribuição: {:?}",
        distribution_strategy
    );

    let task_manager = Arc::new(Mutex::new(TaskManager::new(distribution_strategy)));
    let result_aggregator = Arc::new(Mutex::new(ResultAggregator::new()));

    let ga_config = GAConfig {
        trials: 10,
        max_stagnant: 100,
        generations: 1000,
        tournament_size: 2,
        crossover_probability: 0.9,
        pop_size: None,
    };
    let ag_config_str = serde_json::to_string(&ga_config)?;
    info!("Lendo grafos de: {graphs_path}");
    let paths = fs::read_dir(graphs_path)?;
    let mut tm = task_manager.lock().await;
    for path in paths {
        let path = path?.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                info!("Adicionando tasks para o grafo: {file_name}");
                tm.add_new_graph_tasks(file_name, ga_config.trials, &ag_config_str);
            }
        }
    }
    let total_tasks = tm.get_total_tasks();
    info!("Total de {} tarefas adicionadas.", total_tasks);
    drop(tm);

    let addr_clone = bind_addr.clone();
    tokio::spawn(async move {
        listen_for_workers(addr_clone).await;
    });

    if let Some(path) = save_path {
        let interval = save_interval.unwrap_or(300);
        periodic_saver::start(Arc::clone(&result_aggregator), path.clone(), interval);
    } else {
        warn!("Salvamento periódico desativado.");
    }

    let server_task_manager = Arc::clone(&task_manager);
    let server_result_aggregator = Arc::clone(&result_aggregator);
    let server_bind_addr = bind_addr.clone();
    tokio::spawn(async move {
        info!("Host TCP escutando em {}", server_bind_addr);
        if let Err(e) = start_server(
            &server_bind_addr,
            server_task_manager,
            server_result_aggregator,
        )
        .await
        {
            error!("Erro crítico no servidor: {}", e);
            process::exit(1);
        }
    });

    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;

        let tm_guard = task_manager.lock().await;
        let completed_count = tm_guard.get_completed_tasks_count();

        info!(
            "Progresso: {}/{} tarefas concluídas.",
            completed_count, total_tasks
        );

        if completed_count >= total_tasks {
            info!("Todas as tarefas foram concluídas!");
            let ra_guard = result_aggregator.lock().await;
            if let Err(e) = ra_guard.generate_and_save_report(&tm_guard, report_path) {
                error!("Falha ao gerar o relatório final: {}", e);
            } else {
                info!("Relatório final salvo com sucesso em '{}'", report_path);
            }
            break;
        }
    }

    info!("Encerrando o host...");
    Ok(())
}
