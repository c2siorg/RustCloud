use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureVMScaleSetClient {
    client: Client,
    subscription_id: String,
}

impl AzureVMScaleSetClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        AzureVMScaleSetClient {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_vmss(
        &self,
        resource_group: &str,
        vmss_name: &str,
        location: &str,
        subnet_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            vmss_name
        );

        let body = serde_json::json!({
            "location": location,
            "sku": {
                "name": "Standard_B1s",
                "capacity": 2,
                "tier": "Standard"
            },
            "properties": {
                "upgradePolicy": {
                    "mode": "Manual"
                },
                "virtualMachineProfile": {
                    "storageProfile": {
                        "imageReference": {
                            "publisher": "Canonical",
                            "offer": "0001-com-ubuntu-server-jammy",
                            "sku": "22_04-lts",
                            "version": "latest"
                        }
                    },
                    "osProfile": {
                        "computerNamePrefix": vmss_name,
                        "adminUsername": "azureuser",
                        "adminPassword": "Password1234!"
                    },
                    "networkProfile": {
                        "networkInterfaceConfigurations": [
                            {
                                "name": "vmss-nic",
                                "properties": {
                                    "primary": true,
                                    "ipConfigurations": [
                                        {
                                            "name": "vmss-ipconfig",
                                            "properties": {
                                                "subnet": {
                                                    "id": subnet_id
                                                }
                                            }
                                        }
                                    ]
                                }
                            }
                        ]
                    }
                }
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
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE VMSS API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure VM Scale Set creation failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn list_vmss(&self, resource_group: &str) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets?api-version=2023-09-01",
            self.subscription_id,
            resource_group
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE VMSS API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure list VM Scale Sets failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn get_vmss(
        &self,
        resource_group: &str,
        vmss_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            vmss_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE VMSS API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure get VM Scale Set failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn delete_vmss(
        &self,
        resource_group: &str,
        vmss_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            vmss_name
        );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE VMSS API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure VM Scale Set deletion failed: {}", body_text).into());
        }

        Ok(())
    }
}
