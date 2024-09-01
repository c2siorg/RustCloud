use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleStorage {
    pub name: String,
    pub r#type: String,
    pub zone: String,
    pub size_gb: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    pub name: String,
    pub creation_timestamp: String,
    pub description: String,
    pub disk_size_gb: String,
    pub id: String,
    pub kind: String,
    pub label_fingerprint: String,
    pub self_link: String,
    pub source_disk: String,
    pub source_disk_id: String,
    pub status: String,
    pub storage_bytes: String,
    pub storage_bytes_status: String,
    pub licenses: Vec<String>,
    pub source_disk_encryption_keys: SourceDiskEncryptionKey,
    pub snapshot_encryption_keys: SnapshotEncryptionKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceDiskEncryptionKey {
    pub raw_key: String,
    pub sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnapshotEncryptionKey {
    pub raw_key: String,
    pub sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttachDisk {
    pub source: String,
    pub device_name: String,
    pub auto_delete: bool,
    pub boot: bool,
    pub disk_encryption_keys: DiskEncryptionKey,
    pub index: i32,
    pub interface: String,
    pub kind: String,
    pub licenses: Vec<String>,
    pub mode: String,
    pub r#type: String,
    pub initialize_params: InitializeParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeParams {
    pub disk_name: String,
    pub disk_type: String,
    pub disk_size_gb: String,
    pub source_image: String,
    pub source_image_encryption_keys: SourceImageEncryptionKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Creatdisk {
    pub name: String,
    pub r#type: String,
    pub zone: String,
    pub size_gb: String,
    pub source_image_encryption_keys: SourceImageEncryptionKey,
    pub disk_encryption_keys: DiskEncryptionKey,
    pub source_snapshot_encryption_keys: SourceSnapshotEncryptionKey,
    pub licenses: Vec<String>,
    pub users: Vec<String>,
    pub creation_timestamp: String,
    pub desription: String,
    pub id: String,
    pub kind: String,
    pub label_fingerprint: String,
    pub source_snapshot_id: String,
    pub status: String,
    pub last_attach_timestamp: String,
    pub last_detach_timestamp: String,
    pub options: String,
    pub self_link: String,
    pub source_image: String,
    pub source_image_id: String,
    pub source_snapshot: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceImageEncryptionKey {
    pub raw_key: String,
    pub sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiskEncryptionKey {
    pub raw_key: String,
    pub sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceSnapshotEncryptionKey {
    pub raw_key: String,
    pub sha256: String,
}
