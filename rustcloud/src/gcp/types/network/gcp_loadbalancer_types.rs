use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPools {
    pub name: String,
    pub health_checks: Vec<String>,
    pub description: String,
    pub backup_pool: String,
    pub failover_ratio: i32,
    pub id: String,
    pub instances: Vec<String>,
    pub kind: String,
    pub session_affinity: String,
    pub region: String,
    pub self_link: String,
    pub creation_timestamp: String,
}
