use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureFunctionsClient {
    client: Client,
    subscription_id: String,
}

impl AzureFunctionsClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        AzureFunctionsClient {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_function_app(
        &self,
        resource_group: &str,
        function_name: &str,
        location: &str,
        app_service_plan: &str,
        storage_account: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}?api-version=2022-09-01",
            self.subscription_id,
            resource_group,
            function_name
        );

        let body = serde_json::json!({
            "location": location,
            "kind": "functionapp",
            "properties": {
                "serverFarmId": app_service_plan,
                "siteConfig": {
                    "appSettings": [
                        {
                            "name": "AzureWebJobsStorage",
                            "value": storage_account
                        },
                        {
                            "name": "FUNCTIONS_WORKER_RUNTIME",
                            "value": "node"
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
            return Err(format!("Azure function creation failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn list_function_apps(
        &self,
        resource_group: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites?api-version=2022-09-01",
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
            return Err(format!("Azure list functions failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn get_function_app(
        &self,
        resource_group: &str,
        function_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}?api-version=2022-09-01",
            self.subscription_id,
            resource_group,
            function_name
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
            return Err(format!("Azure get function failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn list_functions_in_app(
        &self,
        resource_group: &str,
        function_app: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}/functions?api-version=2022-09-01",
        self.subscription_id,
        resource_group,
        function_app
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
            return Err(format!("Azure list functions failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn restart_function_app(
        &self,
        resource_group: &str,
        function_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}/restart?api-version=2022-09-01",
            self.subscription_id,
            resource_group,
            function_name
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
            return Err(format!("Azure restart function failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn delete_function_app(
        &self,
        resource_group: &str,
        function_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}?api-version=2022-09-01",
            self.subscription_id,
            resource_group,
            function_name
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
            return Err(format!("Azure function deletion failed: {}", body_text).into());
        }

        Ok(())
    }
}
