use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDatasetRequest {
    pub parent: String,
    pub dataset: Dataset,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    pub display_name: String,
    pub tables_dataset_metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDatasetRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportDataSetRequest {
    pub name: String,
    pub input_config: InputConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputConfig {
    pub gcs_source: GcsSource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GcsSource {
    pub input_uris: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsRequest {
    pub parent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateModelRequest {
    pub parent: String,
    pub model: Model,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub dataset_id: String,
    pub display_name: String,
    pub tables_model_metadata: TablesModelMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TablesModelMetadata {
    pub target_column_spec: TargetColumnSpec,
    pub train_budget_milli_node_hours: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetColumnSpec {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployModelRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UndeployModelRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetModelRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportDatasetRequest {
    pub name: String,
    pub output_config: OutputConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    pub gcs_destination: GcsDestination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GcsDestination {
    pub output_uri_prefix: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteModelRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDatasetRequest {
    pub name: String,
}
