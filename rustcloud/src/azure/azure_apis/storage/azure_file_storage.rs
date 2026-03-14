use crate::azure::azure_apis::auth::azure_storage_auth::AzureStorageAuth;
use reqwest::Client;

pub struct AzureFileClient {
    client: Client,
    account: String,
}

impl AzureFileClient {
    pub fn new(account: String) -> Self {
        Self {
            client: Client::new(),
            account,
        }
    }

    fn base_url(&self) -> String {
        format!("https://{}.file.core.windows.net", self.account)
    }

    pub async fn list_shares(&self) -> Result<String, reqwest::Error> {
        let resource = "/?comp=list";

        let (auth, date) =
            AzureStorageAuth::generate_headers("GET", &self.account, resource, None, None);

        let url = format!("{}?comp=list", self.base_url());

        let res = self
            .client
            .get(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST SHARES");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        Ok(body)
    }

    pub async fn create_share(&self, share: &str) -> Result<String, reqwest::Error> {
        let resource = format!("/{}?restype=share", share);

        let (auth, date) =
            AzureStorageAuth::generate_headers("PUT", &self.account, &resource, Some(0), None);

        let url = format!("{}/{}?restype=share", self.base_url(), share);

        let res = self
            .client
            .put(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .header("Content-Length", "0")
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE CREATE SHARE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        Ok(body)
    }

    pub async fn delete_share(&self, share: &str) -> Result<String, reqwest::Error> {
        let resource = format!("/{}?restype=share", share);

        let (auth, date) =
            AzureStorageAuth::generate_headers("DELETE", &self.account, &resource, None, None);

        let url = format!("{}/{}?restype=share", self.base_url(), share);

        let res = self
            .client
            .delete(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE SHARE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        Ok(body)
    }

    pub async fn get_share_properties(&self, share: &str) -> Result<String, reqwest::Error> {
        let resource = format!("/{}?restype=share", share);

        let (auth, date) =
            AzureStorageAuth::generate_headers("HEAD", &self.account, &resource, None, None);

        let url = format!("{}/{}?restype=share", self.base_url(), share);

        let res = self
            .client
            .head(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .send()
            .await?;

        let status = res.status();

        println!("AZURE GET SHARE PROPERTIES");
        println!("Status  : {}", status);

        Ok(status.to_string())
    }

    pub async fn list_files(&self, share: &str, directory: &str) -> Result<String, reqwest::Error> {
        let resource = format!("/{}/{}?restype=directory&comp=list", share, directory);

        let (auth, date) =
            AzureStorageAuth::generate_headers("GET", &self.account, &resource, None, None);

        let url = format!(
            "{}/{}/{}?restype=directory&comp=list",
            self.base_url(),
            share,
            directory
        );

        let res = self
            .client
            .get(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST FILES");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        Ok(body)
    }

    pub async fn create_directory(
        &self,
        share: &str,
        directory: &str,
    ) -> Result<String, reqwest::Error> {
        let resource = format!("/{}/{}?restype=directory", share, directory);

        let extra_headers = vec![
            ("x-ms-file-attributes", "Directory"),
            ("x-ms-file-permission", "inherit"),
            ("x-ms-file-creation-time", "now"),
            ("x-ms-file-last-write-time", "now"),
        ];

        let (auth, date) = AzureStorageAuth::generate_headers(
            "PUT",
            &self.account,
            &resource,
            Some(0),
            Some(extra_headers),
        );

        let url = format!(
            "{}/{}/{}?restype=directory",
            self.base_url(),
            share,
            directory
        );

        let res = self
            .client
            .put(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .header("x-ms-file-attributes", "Directory")
            .header("x-ms-file-permission", "inherit")
            .header("x-ms-file-creation-time", "now")
            .header("x-ms-file-last-write-time", "now")
            .header("Content-Length", "0")
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE CREATE DIRECTORY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        Ok(body)
    }

    pub async fn delete_directory(
        &self,
        share: &str,
        directory: &str,
    ) -> Result<String, reqwest::Error> {
        let resource = format!("/{}/{}?restype=directory", share, directory);

        let (auth, date) =
            AzureStorageAuth::generate_headers("DELETE", &self.account, &resource, None, None);

        let url = format!(
            "{}/{}/{}?restype=directory",
            self.base_url(),
            share,
            directory
        );

        let res = self
            .client
            .delete(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE DIRECTORY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        Ok(body)
    }

    pub async fn create_file(
        &self,
        share: &str,
        directory: &str,
        file: &str,
        size: u64,
    ) -> Result<String, reqwest::Error> {
        let resource = format!("/{}/{}/{}", share, directory, file);

        let size_str = size.to_string();

        let extra_headers = vec![
            ("x-ms-type", "file"),
            ("x-ms-content-length", size_str.as_str()),
            ("x-ms-file-permission", "inherit"),
            ("x-ms-file-attributes", "Archive"),
            ("x-ms-file-creation-time", "now"),
            ("x-ms-file-last-write-time", "now"),
        ];

        let (auth, date) = AzureStorageAuth::generate_headers(
            "PUT",
            &self.account,
            &resource,
            Some(0),
            Some(extra_headers.clone()),
        );

        let url = format!("{}/{}/{}/{}", self.base_url(), share, directory, file);

        let res = self
            .client
            .put(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .header("x-ms-type", "file")
            .header("x-ms-content-length", size)
            .header("x-ms-file-permission", "inherit")
            .header("x-ms-file-attributes", "Archive")
            .header("x-ms-file-creation-time", "now")
            .header("x-ms-file-last-write-time", "now")
            .header("Content-Length", "0")
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE CREATE FILE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        Ok(body)
    }

    pub async fn upload_file_range(
        &self,
        share: &str,
        directory: &str,
        file: &str,
        content: Vec<u8>,
    ) -> Result<String, reqwest::Error> {
        let len = content.len();
        let range = format!("bytes=0-{}", len - 1);

        let resource = format!("/{}/{}/{}?comp=range", share, directory, file);

        let extra_headers = vec![("x-ms-range", range.as_str()), ("x-ms-write", "update")];

        let (auth, date) = AzureStorageAuth::generate_headers(
            "PUT",
            &self.account,
            &resource,
            Some(len),
            Some(extra_headers.clone()),
        );

        let url = format!(
            "{}/{}/{}/{}?comp=range",
            self.base_url(),
            share,
            directory,
            file
        );

        let res = self
            .client
            .put(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .header("x-ms-range", range)
            .header("x-ms-write", "update")
            .body(content)
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE UPLOAD FILE RANGE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        Ok(body)
    }

    pub async fn download_file(
        &self,
        share: &str,
        directory: &str,
        file: &str,
    ) -> Result<String, reqwest::Error> {
        let resource = format!("/{}/{}/{}", share, directory, file);

        let (auth, date) =
            AzureStorageAuth::generate_headers("GET", &self.account, &resource, None, None);

        let url = format!("{}/{}/{}/{}", self.base_url(), share, directory, file);

        let res = self
            .client
            .get(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DOWNLOAD FILE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        Ok(body)
    }

    pub async fn get_file_properties(
        &self,
        share: &str,
        directory: &str,
        file: &str,
    ) -> Result<String, reqwest::Error> {
        let resource = format!("/{}/{}/{}", share, directory, file);

        let (auth, date) =
            AzureStorageAuth::generate_headers("HEAD", &self.account, &resource, None, None);

        let url = format!("{}/{}/{}/{}", self.base_url(), share, directory, file);

        let res = self
            .client
            .head(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .send()
            .await?;

        let status = res.status();

        println!("AZURE GET FILE PROPERTIES");
        println!("Status  : {}", status);

        for (key, value) in res.headers() {
            println!("{}: {:?}", key, value);
        }

        Ok(status.to_string())
    }

    pub async fn delete_file(
        &self,
        share: &str,
        directory: &str,
        file: &str,
    ) -> Result<String, reqwest::Error> {
        let resource = format!("/{}/{}/{}", share, directory, file);

        let (auth, date) =
            AzureStorageAuth::generate_headers("DELETE", &self.account, &resource, None, None);

        let url = format!("{}/{}/{}/{}", self.base_url(), share, directory, file);

        let res = self
            .client
            .delete(url)
            .header("Authorization", auth)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE FILE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        Ok(body)
    }
}
