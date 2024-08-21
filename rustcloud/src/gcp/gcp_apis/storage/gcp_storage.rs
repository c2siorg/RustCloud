use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::collections::HashMap;

struct GoogleStorage {
    client: reqwest::Client,
    base_url: String,
}

impl GoogleStorage {
    fn new() -> Self {
        GoogleStorage {
            client: reqwest::Client::new(),
            base_url: "https://www.googleapis.com/compute/v1".to_string(),
        }
    }

    async fn create_disk(
        &self,
        request: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let mut option = HashMap::new();
        let mut project_id = String::new();
        let mut zone = String::new();
        let mut disk_type = String::new();

        for (key, value) in request.iter() {
            match key.as_str() {
                "projectid" => project_id = value.as_str().unwrap_or_default().to_string(),
                "Name" => {
                    option.insert("Name", value);
                }
                "Zone" => zone = value.as_str().unwrap_or_default().to_string(),
                "Type" => disk_type = value.as_str().unwrap_or_default().to_string(),
                "SizeGb" => {
                    option.insert("SizeGb", value);
                }
                "SourceImageEncryptionKeys" => {
                    option.insert("SourceImageEncryptionKeys", value);
                }
                "DiskEncryptionKeys" => {
                    option.insert("DiskEncryptionKeys", value);
                }
                "SourceSnapshotEncryptionKeys" => {
                    option.insert("SourceSnapshotEncryptionKeys", value);
                }
                "Licenses" => {
                    option.insert("Licenses", value);
                }
                "Users" => {
                    option.insert("Users", value);
                }
                "CreationTimestamp" => {
                    option.insert("CreationTimestamp", value);
                }
                "Description" => {
                    option.insert("Description", value);
                }
                "ID" => {
                    option.insert("ID", value);
                }
                "Kind" => {
                    option.insert("Kind", value);
                }
                "LabelFingerprint" => {
                    option.insert("LabelFingerprint", value);
                }
                "SourceSnapshotID" => {
                    option.insert("SourceSnapshotID", value);
                }
                "Status" => {
                    option.insert("Status", value);
                }
                "LastAttachTimestamp" => {
                    option.insert("LastAttachTimestamp", value);
                }
                "LastDetachTimestamp" => {
                    option.insert("LastDetachTimestamp", value);
                }
                "Options" => {
                    option.insert("Options", value);
                }
                "SelfLink" => {
                    option.insert("SelfLink", value);
                }
                "SourceImage" => {
                    option.insert("SourceImage", value);
                }
                "SourceImageID" => {
                    option.insert("SourceImageID", value);
                }
                "SourceSnapshot" => {
                    option.insert("SourceSnapshot", value);
                }
                _ => {}
            }
        }

        option.insert(
            "Zone",
            &Value::String(format!("projects/{}/zones/{}", project_id, zone)),
        );
        option.insert(
            "Type",
            &Value::String(format!(
                "projects/{}/zones/{}/diskTypes/{}",
                project_id, zone, disk_type
            )),
        );

        let create_disk_json = serde_json::to_string(&option).unwrap();
        let url = format!(
            "{}/projects/{}/zones/{}/disks",
            self.base_url, project_id, zone
        );
        let token = retrieve_token().await.unwrap();

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, token)
            .body(create_disk_json)
            .send()
            .await?;

        let mut body = String::new();
        let mut response: HashMap<String, Value> = HashMap::new();
        response.insert(
            "status".to_string(),
            Value::Number(resp.status().as_u16().into()),
        );
        response.insert("body".to_string(), Value::String(body));

        Ok(response)
    }

    async fn delete_disk(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let url = format!(
            "{}/projects/{}/zones/{}/disks/{}",
            self.base_url, request["projectid"], request["Zone"], request["disk"]
        );
        let token = retrieve_token().await.unwrap();
        let resp = self
            .client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, token)
            .send()
            .await?;

        let mut body = String::new();

        let mut response = HashMap::new();
        response.insert(
            "status".to_string(),
            Value::Number(resp.status().as_u16().into()),
        );
        response.insert("body".to_string(), Value::String(body));

        Ok(response)
    }

    async fn create_snapshot(
        &self,
        request: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let mut snapshot = HashMap::new();
        let mut project_id = String::new();
        let mut zone = String::new();
        let mut disk = String::new();

        for (key, value) in request.iter() {
            match key.as_str() {
                "projectid" => project_id = value.as_str().unwrap_or_default().to_string(),
                "Name" => {
                    snapshot.insert("Name", value);
                }
                "Zone" => zone = value.as_str().unwrap_or_default().to_string(),
                "disk" => disk = value.as_str().unwrap_or_default().to_string(),
                "CreationTimestamp" => {
                    snapshot.insert("CreationTimestamp", value);
                }
                "Description" => {
                    snapshot.insert("Description", value);
                }
                "DiskSizeGb" => {
                    snapshot.insert("DiskSizeGb", value);
                }
                "ID" => {
                    snapshot.insert("ID", value);
                }
                "Kind" => {
                    snapshot.insert("Kind", value);
                }
                "LabelFingerprint" => {
                    snapshot.insert("LabelFingerprint", value);
                }
                "SelfLink" => {
                    snapshot.insert("SelfLink", value);
                }
                "StorageBytes" => {
                    snapshot.insert("StorageBytes", value);
                }
                "Status" => {
                    snapshot.insert("Status", value);
                }
                "SourceDiskID" => {
                    snapshot.insert("SourceDiskID", value);
                }
                "SourceDisk" => {
                    snapshot.insert("SourceDisk", value);
                }
                "StorageBytesStatus" => {
                    snapshot.insert("StorageBytesStatus", value);
                }
                "Licenses" => {
                    snapshot.insert("Licenses", value);
                }
                "SourceDiskEncryptionKeys" => {
                    snapshot.insert("SourceDiskEncryptionKeys", value);
                }
                "SnapshotEncryptionKeys" => {
                    snapshot.insert("SnapshotEncryptionKeys", value);
                }
                _ => {}
            }
        }

        let create_snapshot_json = serde_json::to_string(&snapshot).unwrap();
        let url = format!(
            "{}/projects/{}/zones/{}/disks/{}/createSnapshot",
            self.base_url, project_id, zone, disk
        );
        let token = retrieve_token().await.unwrap();

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, token)
            .body(create_snapshot_json)
            .send()
            .await?;

        let mut body = String::new();

        let mut response = HashMap::new();
        response.insert(
            "status".to_string(),
            Value::Number(resp.status().as_u16().into()),
        );
        response.insert("body".to_string(), Value::String(body));

        Ok(response)
    }
}
