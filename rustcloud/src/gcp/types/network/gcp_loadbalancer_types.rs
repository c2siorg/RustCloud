use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPools {
    pub name: Option<String>,
    #[serde(rename = "healthChecks")]
    pub health_checks: Option<Vec<String>>,
    pub description: Option<String>,
    #[serde(rename = "backupPool")]
    pub backup_pool: Option<String>,
    #[serde(rename = "failoverRatio")]
    pub failover_ratio: Option<f64>,
    pub id: Option<String>,
    pub instances: Option<Vec<String>>,
    pub kind: Option<String>,
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: Option<String>,
    pub region: Option<String>,
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: Option<String>,
}
