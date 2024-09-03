use serde::{Deserialize, Serialize};
use serde_json::Value;

// Represents Google Container attributes and methods

// Represents the attributes of CreateCluster
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateCluster {
    pub name: String,
    pub zone: String,
    pub network: String,
    #[serde(rename = "loggingService")]
    pub logging_service: String,
    #[serde(rename = "monitoringService")]
    pub monitoring_service: String,
    #[serde(rename = "initialClusterVersion")]
    pub initial_cluster_version: String,
    pub subnetwork: String,
    #[serde(rename = "legacyAbac")]
    pub legacy_abac: LegacyAbac,
    #[serde(rename = "masterAuth")]
    pub master_auth: MasterAuth,
    #[serde(rename = "nodePools")]
    pub node_pools: Vec<NodePool>,
}

// Represents the legacyAbac attributes of CreateCluster
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LegacyAbac {
    pub enabled: bool,
}

// Represents the masterAuth attributes of CreateCluster
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MasterAuth {
    pub username: String,
    #[serde(rename = "clientCertificateConfig")]
    pub client_certificate_config: ClientCertificateConfigs,
}

// Represents the ClientCertificateConfigs attributes of MasterAuth
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClientCertificateConfigs {
    #[serde(rename = "issueClientCertificate")]
    pub issue_client_certificate: bool,
}

// Represents the config attributes of NodePool
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "machineType")]
    pub machine_type: String,
    #[serde(rename = "imageType")]
    pub image_type: String,
    #[serde(rename = "diskSizeGb")]
    pub disk_size_gb: i32,
    pub preemptible: bool,
    #[serde(rename = "oauthScopes")]
    pub oauth_scopes: Vec<String>,
}

// Represents the autoscaling attributes of NodePool
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Autoscaling {
    pub enabled: bool,
    #[serde(rename = "minNodeCount")]
    pub min_node_count: i32,
    #[serde(rename = "maxNodeCount")]
    pub max_node_count: i32,
}

// Represents the management attributes of NodePool
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Management {
    #[serde(rename = "autoUpgrade")]
    pub auto_upgrade: bool,
    #[serde(rename = "autoRepair")]
    pub auto_repair: bool,
}

// Represents NodePool attributes of CreateCluster
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NodePool {
    pub name: String,
    #[serde(rename = "initialNodeCount")]
    pub initial_node_count: i64,
    pub config: Config,
    pub autoscaling: Autoscaling,
    pub management: Management,
}

// Represents NodePool attributes of CreateService
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NodePoolService {
    #[serde(rename = "config")]
    pub config: ConfigService,
    pub name: String,
    #[serde(rename = "statusMessage")]
    pub status_message: String,
    pub autoscaling: AutoscalingService,
    #[serde(rename = "initialNodeCount")]
    pub initial_node_count: i32,
    pub management: ManagementService,
    #[serde(rename = "selfLink")]
    pub self_link: String,
    pub version: String,
    #[serde(rename = "instanceGroupUrls")]
    pub instance_group_urls: Vec<String>,
    pub status: String,
}

// Represents config attributes of NodePool for CreateService
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigService {
    #[serde(rename = "machineType")]
    pub machine_type: String,
    #[serde(rename = "imageType")]
    pub image_type: String,
    #[serde(rename = "oauthScopes")]
    pub oauth_scopes: Vec<String>,
    pub preemptible: bool,
    #[serde(rename = "localSsdCount")]
    pub local_ssd_count: i32,
    #[serde(rename = "diskSizeGb")]
    pub disk_size_gb: i32,
    #[serde(rename = "serviceAccount")]
    pub service_account: String,
}

// Represents autoscaling attributes of NodePool for CreateService
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoscalingService {
    #[serde(rename = "maxNodeCount")]
    pub max_node_count: i32,
    #[serde(rename = "minNodeCount")]
    pub min_node_count: i32,
    pub enabled: bool,
}

// Represents management attributes of NodePool for CreateService
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ManagementService {
    #[serde(rename = "autoRepair")]
    pub auto_repair: bool,
    #[serde(rename = "autoUpgrade")]
    pub auto_upgrade: bool,
}
