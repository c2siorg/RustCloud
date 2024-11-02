use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::collections::HashMap;

pub struct GoogleStorage {
    client: reqwest::Client,
    base_url: String,
}

impl GoogleStorage {
    pub fn new() -> Self {
        GoogleStorage {
            client: reqwest::Client::new(),
            base_url: "https://www.googleapis.com/compute/v1".to_string(),
        }
    }

    pub async fn create_disk(
        &self,
        request: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        let mut option = HashMap::new();
        let mut project_id = String::new();
        let mut zone = String::new();
        let mut disk_type = String::new();

        for (key, value) in request.iter() {
            match key.as_str() {
                "projectid" => project_id = value.as_str().unwrap_or_default().to_string(),
                "Name" => {
                    option.insert("name", value);
                }
                "Zone" => zone = value.as_str().unwrap_or_default().to_string(),
                "Type" => disk_type = value.as_str().unwrap_or_default().to_string(),
                "SizeGb" => {
                    option.insert("sizeGb", value);
                }
                "SourceImageEncryptionKeys" => {
                    option.insert("sourceImageEncryptionKey", value);
                }
                "DiskEncryptionKeys" => {
                    option.insert("diskEncryptionKey", value);
                }
                "SourceSnapshotEncryptionKeys" => {
                    option.insert("sourceSnapshotEncryptionKey", value);
                }
                "Licenses" => {
                    option.insert("licenses", value);
                }
                "Users" => {
                    option.insert("users", value);
                }
                "CreationTimestamp" => {
                    option.insert("creationTimestamp", value);
                }
                "Description" => {
                    option.insert("description", value);
                }
                "ID" => {
                    option.insert("id", value);
                }
                "Kind" => {
                    option.insert("kind", value);
                }
                "LabelFingerprint" => {
                    option.insert("labelFingerprint", value);
                }
                "SourceSnapshotID" => {
                    option.insert("sourceSnapshotID", value);
                }
                "Status" => {
                    option.insert("status", value);
                }
                "LastAttachTimestamp" => {
                    option.insert("lastAttachTimestamp", value);
                }
                "LastDetachTimestamp" => {
                    option.insert("lastDetachTimestamp", value);
                }
                "Options" => {
                    option.insert("options", value);
                }
                "SelfLink" => {
                    option.insert("selfLink", value);
                }
                "SourceImage" => {
                    option.insert("sourceImage", value);
                }
                "SourceImageID" => {
                    option.insert("sourceImageID", value);
                }
                "SourceSnapshot" => {
                    option.insert("sourceSnapshot", value);
                }
                _ => {}
            }
        }

        let Zone = Value::String(format!("projects/{}/zones/{}", project_id, zone));
        let Type = Value::String(format!(
            "projects/{}/zones/{}/diskTypes/{}",
            project_id.clone(), zone.clone(), disk_type.clone()
        ));
        option.insert(
            "zone",
            &Zone,
        );

        option.insert(
            "type",
            &Type,
        );

        let create_disk_json = serde_json::to_string(&option).unwrap();
        let url = format!(
            "{}/projects/{}/zones/{}/disks",
            self.base_url, project_id.clone(), zone.clone()
        );
        let token = retrieve_token().await.map_err(|e| format!("Failed to retrieve token: {}", e))?;

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(create_disk_json)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;


        let status = resp.status();
        if !status.is_success() {
            let response_text = resp.text().await?;
            println!("{:?}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        // Parse the response body
        let body = resp
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
    
        println!("{:?}", body);

        let mut response: HashMap<String, Value> = HashMap::new();
        response.insert(
            "status".to_string(),
            Value::Number(status.as_u16().into()),
        );
        response.insert("body".to_string(), Value::String(body));

        Ok(response)
    }

    pub async fn delete_disk(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/zones/{}/disks/{}",
            self.base_url, request["projectid"], request["Zone"], request["disk"]
        );
        let token = retrieve_token().await.map_err(|e| format!("Failed to retrieve token: {}", e))?;
        let resp = self
            .client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;


        let status = resp.status();
        if !status.is_success() {
            let response_text = resp.text().await?;
            println!("{:?}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        // Parse the response body
        let body = resp
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
    
        println!("{:?}", body);

        let mut response = HashMap::new();
        response.insert(
            "status".to_string(),
            Value::Number(status.as_u16().into()),
        );
        response.insert("body".to_string(), Value::String(body));

        Ok(response)
    }

    pub async fn create_snapshot(
        &self,
        request: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        let mut snapshot = HashMap::new();
        let mut project_id = String::new();
        let mut zone = String::new();
        let mut disk = String::new();

        for (key, value) in request.iter() {
            match key.as_str() {
                "projectid" => project_id = value.as_str().unwrap_or_default().to_string(),
                "Name" => {
                    snapshot.insert("name", value);
                }
                "Zone" => zone = value.as_str().unwrap_or_default().to_string(),
                "disk" => disk = value.as_str().unwrap_or_default().to_string(),
                "CreationTimestamp" => {
                    snapshot.insert("creationTimestamp", value);
                }
                "Description" => {
                    snapshot.insert("description", value);
                }
                "DiskSizeGb" => {
                    snapshot.insert("diskSizeGb", value);
                }
                "ID" => {
                    snapshot.insert("id", value);
                }
                "Kind" => {
                    snapshot.insert("kind", value);
                }
                "LabelFingerprint" => {
                    snapshot.insert("labelFingerprint", value);
                }
                "SelfLink" => {
                    snapshot.insert("selfLink", value);
                }
                "StorageBytes" => {
                    snapshot.insert("storageBytes", value);
                }
                "Status" => {
                    snapshot.insert("status", value);
                }
                "SourceDiskID" => {
                    snapshot.insert("sourceDiskID", value);
                }
                "SourceDisk" => {
                    snapshot.insert("sourceDisk", value);
                }
                "StorageBytesStatus" => {
                    snapshot.insert("storageBytesStatus", value);
                }
                "Licenses" => {
                    snapshot.insert("licenses", value);
                }
                "SourceDiskEncryptionKeys" => {
                    snapshot.insert("sourceDiskEncryptionKey", value);
                }
                "SnapshotEncryptionKeys" => {
                    snapshot.insert("snapshotEncryptionKey", value);
                }
                _ => {}
            }
        }

        let create_snapshot_json = serde_json::to_string(&snapshot).unwrap();
        let url = format!(
            "{}/projects/{}/zones/{}/disks/{}/createSnapshot",
            self.base_url, project_id, zone, disk
        );
        let token = retrieve_token().await.map_err(|e| format!("Failed to retrieve token: {}", e))?;

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(create_snapshot_json)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            let response_text = resp.text().await?;
            println!("{:?}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        // Parse the response body
        let body = resp
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
    
        println!("{:?}", body);

        let mut response = HashMap::new();
        response.insert(
            "status".to_string(),
            Value::Number(status.as_u16().into()),
        );
        response.insert("body".to_string(), Value::String(body));

        Ok(response)
    }
}