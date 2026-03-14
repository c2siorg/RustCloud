use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureApplicationGatewayClient {
    client: Client,
    subscription_id: String,
}

impl AzureApplicationGatewayClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        Self {
            client: Client::new(),
            subscription_id,
        }
    }

    fn gateway_id(&self, resource_group: &str, gateway_name: &str) -> String {
        format!(
        "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/applicationGateways/{}",
        self.subscription_id,
        resource_group,
        gateway_name
    )
    }

    fn gateway_url(&self, resource_group: &str, gateway_name: &str) -> String {
        format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/applicationGateways/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            gateway_name
        )
    }

    pub async fn create_application_gateway(
        &self,
        resource_group: &str,
        gateway_name: &str,
        location: &str,
        subnet_id: &str,
        public_ip_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;
        let gateway_id = self.gateway_id(resource_group, gateway_name);
        let url = self.gateway_url(resource_group, gateway_name);

        let body = serde_json::json!({
                    "location": location,
                    "properties": {
                        "sku": {
                            "name": "Standard_v2",
                            "tier": "Standard_v2",
                            "capacity": 1
                        },

                        "gatewayIPConfigurations": [
                            {
                                "name": "appGatewayIpConfig",
                                "properties": {
                                    "subnet": {
                                        "id": subnet_id
                                    }
                                }
                            }
                        ],

                        "frontendIPConfigurations": [
                            {
                                "name": "appGatewayFrontendIP",
                                "properties": {
                                    "publicIPAddress": {
                                        "id": public_ip_id
                                    }
                                }
                            }
                        ],

                        "frontendPorts": [
                            {
                                "name": "port80",
                                "properties": {
                                    "port": 80
                                }
                            }
                        ],

                        "backendAddressPools": [
                            {
                                "name": "backendPool"
                            }
                        ],

                        "backendHttpSettingsCollection": [
                            {
                                "name": "httpSettings",
                                "properties": {
                                    "port": 80,
                                    "protocol": "Http",
                                    "cookieBasedAffinity": "Disabled"
                                }
                            }
                        ],

                        "httpListeners": [
                            {
                                "name": "listener",
                                "properties": {
                                    "frontendIPConfiguration": {
                                        "id": format!("{}/frontendIPConfigurations/appGatewayFrontendIP", gateway_id)
                                    },
                                    "frontendPort": {
                                        "id": format!("{}/frontendPorts/port80", gateway_id)
                                    },
                                    "protocol": "Http"
                                }
                            }
                        ],

                        "requestRoutingRules": [
        {
            "name": "rule1",
            "properties": {
                "ruleType": "Basic",

                "priority": 100,

                "httpListener": {
                    "id": format!("{}/httpListeners/listener", gateway_id)
                },
                "backendAddressPool": {
                    "id": format!("{}/backendAddressPools/backendPool", gateway_id)
                },
                "backendHttpSettings": {
                    "id": format!("{}/backendHttpSettingsCollection/httpSettings", gateway_id)
                }
            }
        }
        ]
                    }
                });

        let res = self
            .client
            .put(url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE CREATE APPLICATION GATEWAY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Create Application Gateway failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn get_application_gateway(
        &self,
        resource_group: &str,
        gateway_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = self.gateway_url(resource_group, gateway_name);

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE GET APPLICATION GATEWAY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Get Application Gateway failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_application_gateways_rg(
        &self,
        resource_group: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/applicationGateways?api-version=2023-09-01",
            self.subscription_id,
            resource_group
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST APPLICATION GATEWAYS RG");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Application Gateways RG failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_application_gateways_subscription(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Network/applicationGateways?api-version=2023-09-01",
            self.subscription_id
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST APPLICATION GATEWAYS SUBSCRIPTION");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Application Gateways subscription failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn start_application_gateway(
        &self,
        resource_group: &str,
        gateway_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/applicationGateways/{}/start?api-version=2023-09-01",
        self.subscription_id,
        resource_group,
        gateway_name
    );

        let res = self
            .client
            .post(url)
            .bearer_auth(token)
            .json(&serde_json::json!({}))
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE START APPLICATION GATEWAY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Start Application Gateway failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn stop_application_gateway(
        &self,
        resource_group: &str,
        gateway_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/applicationGateways/{}/stop?api-version=2023-09-01",
        self.subscription_id,
        resource_group,
        gateway_name
    );

        let res = self
            .client
            .post(url)
            .bearer_auth(token)
            .json(&serde_json::json!({}))
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE STOP APPLICATION GATEWAY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Stop Application Gateway failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn delete_application_gateway(
        &self,
        resource_group: &str,
        gateway_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = self.gateway_url(resource_group, gateway_name);

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE APPLICATION GATEWAY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete Application Gateway failed: {}", body).into());
        }

        Ok(())
    }
}
