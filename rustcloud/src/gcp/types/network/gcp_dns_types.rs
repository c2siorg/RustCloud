use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDns {
    pub creation_time: String,
    pub description: String,
    pub dns_name: String,
    pub name_servers: Vec<String>,
    pub id: String,
    pub kind: String,
    pub name: String,
    pub name_server_set: String,
}