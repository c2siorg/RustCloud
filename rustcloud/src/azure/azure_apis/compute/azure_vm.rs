use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureVMClient {
    client: Client,
    subscription_id: String,
}

impl AzureVMClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        AzureVMClient {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_vm(
        &self,
        resource_group: &str,
        vm_name: &str,
        location: &str,
        nic_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}?api-version=2023-09-01",
        self.subscription_id,
        resource_group,
        vm_name
    );

        let body = serde_json::json!({
            "location": location,
            "properties": {
                "hardwareProfile": {
                    "vmSize": "Standard_B1s"
                },
                "storageProfile": {
                    "imageReference": {
                        "publisher": "Canonical",
                        "offer": "0001-com-ubuntu-server-jammy",
                        "sku": "22_04-lts",
                        "version": "latest"
                    }
                },
                "osProfile": {
                    "computerName": vm_name,
                    "adminUsername": "azureuser",
                    "adminPassword": "Password1234!"
                },
                "networkProfile": {
                    "networkInterfaces": [
                        {
                            "id": nic_id
                        }
                    ]
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
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure VM Instance creation failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn list_vms(&self, resource_group: &str) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines?api-version=2023-09-01",
            self.subscription_id,
            resource_group
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure list VM instances failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn start_vm(
        &self,
        resource_group: &str,
        vm_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/start?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            vm_name
        );

        let res = self
            .client
            .post(url)
            .bearer_auth(token)
            .header("Content-Length", "0")
            .send()
            .await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure start VM instance failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn stop_vm(
        &self,
        resource_group: &str,
        vm_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/powerOff?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            vm_name
        );

        let res = self
            .client
            .post(url)
            .bearer_auth(token)
            .header("Content-Length", "0")
            .send()
            .await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure stop VM instance failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn get_vm(
        &self,
        resource_group: &str,
        vm_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}?api-version=2023-09-01",
        self.subscription_id,
        resource_group,
        vm_name
    );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure get VM instance details failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn restart_vm(
        &self,
        resource_group: &str,
        vm_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/restart?api-version=2023-09-01",
        self.subscription_id,
        resource_group,
        vm_name
    );

        let res = self
            .client
            .post(url)
            .bearer_auth(token)
            .header("Content-Length", "0")
            .send()
            .await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure VM instance restart failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn delete_vm(
        &self,
        resource_group: &str,
        vm_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}?api-version=2023-09-01",
        self.subscription_id,
        resource_group,
        vm_name
    );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure VM instance deletion failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn vm_instance_view(
        &self,
        resource_group: &str,
        vm_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/instanceView?api-version=2023-09-01",
        self.subscription_id,
        resource_group,
        vm_name
    );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure VM instance view failed: {}", body_text).into());
        }

        Ok(())
    }
}
