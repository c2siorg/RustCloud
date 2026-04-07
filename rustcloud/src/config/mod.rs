use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloudConfig {
    pub aws: Option<AwsConfig>,
    pub gcp: Option<GcpConfig>,
    pub azure: Option<AzureConfig>,
    pub digitalocean: Option<DigitalOceanConfig>,
    pub defaults: Option<DefaultConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AwsConfig {
    pub region: Option<String>,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GcpConfig {
    pub project_id: Option<String>,
    pub credentials_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AzureConfig {
    pub subscription_id: Option<String>,
    pub tenant_id: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DigitalOceanConfig {
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DefaultConfig {
    pub timeout_seconds: Option<u64>,
    pub retry_attempts: Option<u32>,
}

impl CloudConfig {
    pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path).map_err(ConfigError::Io)?;
        Self::from_json(&content)
    }

    pub fn from_json(content: &str) -> Result<Self, ConfigError> {
        serde_json::from_str(content).map_err(ConfigError::Parse)
    }

    pub fn from_env() -> Self {
        Self {
            aws: Some(AwsConfig {
                region: env::var("AWS_REGION").ok(),
                access_key_id: env::var("AWS_ACCESS_KEY_ID").ok(),
                secret_access_key: env::var("AWS_SECRET_ACCESS_KEY").ok(),
            }),
            gcp: Some(GcpConfig {
                project_id: env::var("GCP_PROJECT_ID").ok(),
                credentials_path: env::var("GOOGLE_APPLICATION_CREDENTIALS").ok(),
            }),
            azure: Some(AzureConfig {
                subscription_id: env::var("AZURE_SUBSCRIPTION_ID").ok(),
                tenant_id: env::var("AZURE_TENANT_ID").ok(),
                client_id: env::var("AZURE_CLIENT_ID").ok(),
                client_secret: env::var("AZURE_CLIENT_SECRET").ok(),
            }),
            digitalocean: Some(DigitalOceanConfig {
                token: env::var("DIGITALOCEAN_TOKEN").ok(),
            }),
            defaults: Some(DefaultConfig {
                timeout_seconds: env::var("CLOUD_TIMEOUT_SECONDS")
                    .ok()
                    .and_then(|v| v.parse().ok()),
                retry_attempts: env::var("CLOUD_RETRY_ATTEMPTS")
                    .ok()
                    .and_then(|v| v.parse().ok()),
            }),
        }
    }

    pub fn merge(&mut self, other: CloudConfig) {
        if let (Some(self_aws), Some(other_aws)) = (&mut self.aws, other.aws) {
            if other_aws.region.is_some() {
                self_aws.region = other_aws.region;
            }
            if other_aws.access_key_id.is_some() {
                self_aws.access_key_id = other_aws.access_key_id;
            }
            if other_aws.secret_access_key.is_some() {
                self_aws.secret_access_key = other_aws.secret_access_key;
            }
        }
        
        if let (Some(self_gcp), Some(other_gcp)) = (&mut self.gcp, other.gcp) {
            if other_gcp.project_id.is_some() {
                self_gcp.project_id = other_gcp.project_id;
            }
            if other_gcp.credentials_path.is_some() {
                self_gcp.credentials_path = other_gcp.credentials_path;
            }
        }
        
        if let (Some(self_azure), Some(other_azure)) = (&mut self.azure, other.azure) {
            if other_azure.subscription_id.is_some() {
                self_azure.subscription_id = other_azure.subscription_id;
            }
            if other_azure.tenant_id.is_some() {
                self_azure.tenant_id = other_azure.tenant_id;
            }
            if other_azure.client_id.is_some() {
                self_azure.client_id = other_azure.client_id;
            }
            if other_azure.client_secret.is_some() {
                self_azure.client_secret = other_azure.client_secret;
            }
        }

        if let (Some(self_do), Some(other_do)) = (&mut self.digitalocean, other.digitalocean) {
            if other_do.token.is_some() {
                self_do.token = other_do.token;
            }
        }

        if let (Some(self_def), Some(other_def)) = (&mut self.defaults, other.defaults) {
            if other_def.timeout_seconds.is_some() {
                self_def.timeout_seconds = other_def.timeout_seconds;
            }
            if other_def.retry_attempts.is_some() {
                self_def.retry_attempts = other_def.retry_attempts;
            }
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(serde_json::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "IO error: {}", e),
            ConfigError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(e: serde_json::Error) -> Self {
        ConfigError::Parse(e)
    }
}
