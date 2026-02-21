use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetReference {
    pub project_id: String,
    pub dataset_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDataset {
    pub dataset_reference: DatasetReference,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableReference {
    pub project_id: String,
    pub dataset_id: String,
    pub table_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub fields: Vec<TableField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTable {
    pub table_reference: TableReference,
    pub schema: TableSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunQuery {
    pub query: String,
    #[serde(rename = "useLegacySql")]
    pub use_legacy_sql: bool,
}
