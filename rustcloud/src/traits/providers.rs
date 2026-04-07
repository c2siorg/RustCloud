use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

use crate::errors::CloudError;

#[async_trait]
pub trait ComputeProvider: Send + Sync {
    async fn create_instance(
        &self,
        request: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn delete_instance(
        &self,
        instance_id: &str,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn list_instances(
        &self,
    ) -> Result<HashMap<String, Value>, CloudError>;
}

#[async_trait]
pub trait StorageProvider: Send + Sync {
    async fn create_bucket(
        &self,
        name: &str,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn delete_bucket(
        &self,
        name: &str,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn list_buckets(
        &self,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn upload_object(
        &self,
        bucket: &str,
        key: &str,
        data: Vec<u8>,
        content_type: Option<&str>,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn download_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<Vec<u8>, CloudError>;

    async fn delete_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<HashMap<String, Value>, CloudError>;
}

#[async_trait]
pub trait DatabaseProvider: Send + Sync {
    async fn create_database(
        &self,
        name: &str,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn delete_database(
        &self,
        name: &str,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn list_databases(
        &self,
    ) -> Result<HashMap<String, Value>, CloudError>;
}

#[async_trait]
pub trait DnsProvider: Send + Sync {
    async fn create_record(
        &self,
        domain: &str,
        record_type: &str,
        name: &str,
        value: &str,
        ttl: Option<u32>,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn delete_record(
        &self,
        domain: &str,
        record_id: &str,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn list_records(
        &self,
        domain: &str,
    ) -> Result<HashMap<String, Value>, CloudError>;
}

#[async_trait]
pub trait LoadBalancerProvider: Send + Sync {
    async fn create_load_balancer(
        &self,
        request: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn delete_load_balancer(
        &self,
        lb_id: &str,
    ) -> Result<HashMap<String, Value>, CloudError>;

    async fn list_load_balancers(
        &self,
    ) -> Result<HashMap<String, Value>, CloudError>;
}
