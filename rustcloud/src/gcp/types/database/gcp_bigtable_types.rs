use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// InitialSplits struct represents InitialSplits.
#[derive(Debug, Serialize, Deserialize)]
pub struct InitialSplits {
    pub key: String,
}

// Table struct represents Table.
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: Option<String>,
    #[serde(rename = "clusterStates")]
    pub cluster_states: Option<HashMap<String, ClusterStates>>,
    #[serde(rename = "columnFamilies")]
    pub column_families: Option<HashMap<String, serde_json::Value>>,
    pub granularity: Option<String>,
    #[serde(rename = "restoreInfo")]
    pub restore_info: Option<serde_json::Value>,
    #[serde(rename = "changeStreamConfig")]
    pub change_stream_config: Option<serde_json::Value>,
    #[serde(rename = "deletionProtection")]
    pub deletion_protection: Option<bool>,
    pub stats: Option<serde_json::Value>,
    #[serde(rename = "automatedBackupPolicy")]
    pub automated_backup_policy: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterStates {
    #[serde(rename = "replicationState")]
    pub replication_state: String,
    #[serde(rename = "encryptionInfo")]
    pub encryption_info: Vec<serde_json::Value>,
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
    #[serde(rename = "initialSplits")]
    pub initial_splits: Option<Vec<InitialSplits>>,
}
