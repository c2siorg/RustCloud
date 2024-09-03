use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDns {
    #[serde(rename = "creationTime")]
    pub creation_time: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "dnsName")]
    pub dns_name: Option<String>,
    #[serde(rename = "nameServers")]
    pub name_servers: Option<Vec<String>>,
    pub id: Option<String>,
    pub kind: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "nameServerSet")]
    pub name_server_set: Option<String>,
}
