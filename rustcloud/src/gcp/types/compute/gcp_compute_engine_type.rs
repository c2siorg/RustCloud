use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVMRequest {
    pub name: String,
    pub machine_type: String,
    pub disks: Vec<Disk>,
    pub network_interfaces: Vec<NetworkInterface>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disk {
    pub boot: bool,
    pub auto_delete: bool,
    pub initialize_params: InitializeParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeParams {
    pub source_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub network: String,
}
