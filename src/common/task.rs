use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub graph_id: String,
    pub run_number: String,
    pub ag_config: String,
}

impl Task {
    pub fn new(graph_id: String, run_number: String, ag_config: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            graph_id,
            run_number,
            ag_config,
        }
    }
}
