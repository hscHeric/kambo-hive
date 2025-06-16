use std::{collections::HashMap, error::Error};

use log::info;

use crate::common::TaskResult;

pub struct ResultAggregator {
    results_by_graph: HashMap<String, Vec<TaskResult>>,
    total_results_collected: usize,
}

impl ResultAggregator {
    pub fn new() -> Self {
        ResultAggregator {
            results_by_graph: HashMap::new(),
            total_results_collected: 0,
        }
    }
    pub fn add_result(&mut self, result: TaskResult) -> Result<(), Box<dyn Error>> {
        let graph_id = result.graph_id.clone();
        self.results_by_graph
            .entry(graph_id)
            .or_default()
            .push(result);

        self.total_results_collected += 1;
        info!(
            "Resultado adicionado. total de resultados: {}",
            self.total_results_collected
        );

        // TODO: Adicionar lógica para verificar se todas as N execuções de um grafo foram concluídas.
        // Se sim, disparar o salvamento em arquivo (ex: csv, JSON, etc.).
        // Você precisaria de um mapa para saber quantas execuções são esperadas por graph_id.
        Ok(())
    }

    pub fn get_results_collected(&self) -> usize {
        self.total_results_collected
    }
}
