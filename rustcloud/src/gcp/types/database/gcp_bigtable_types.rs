use serde::{Deserialize, Serialize};



// InitialSplits struct represents InitialSplits.
#[derive(Debug, Serialize, Deserialize)]
pub struct InitialSplits {
    pub key: String,
}

// Table struct represents Table.
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub granularity: String,
    pub name: String,
}

// ClusterStates struct represents ClusterStates.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterStates {
    pub replication_state: String,
}

// GcRule struct represents GcRule.
#[derive(Debug, Serialize, Deserialize)]
pub struct GcRule {
    pub max_num_versions: i32,
    pub max_age: String,
}

// CreateBigtable struct represents Create Bigtable.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBigtable {
    pub table_id: String,
    pub table: Table,
    pub initial_splits: Vec<InitialSplits>,
    pub cluster_states: ClusterStates,
}
