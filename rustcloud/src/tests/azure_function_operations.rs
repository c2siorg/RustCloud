use crate::azure::azure_apis::compute::azure_function::AzureFunctionsClient;

#[tokio::test]
async fn test_create_function_app() {
    let client = AzureFunctionsClient::new();

    let result = client
        .create_function_app(
            "test-rg",
            "test-function-rc",
            "eastasia",
            "test-app-service-plan",
            "DefaultEndpointsProtocol=teststoragerc;",
        )
        .await;

    match &result {
        Ok(_) => println!("Function app created successfully"),
        Err(e) => println!("Azure function creation failed: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}

#[tokio::test]
async fn test_list_function_apps() {
    let client = AzureFunctionsClient::new();

    let result = client.list_function_apps("test-rg").await;

    match &result {
        Ok(_) => println!("Functions listed successfully"),
        Err(e) => println!("Azure list functions failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_get_function_app() {
    let client = AzureFunctionsClient::new();

    let result = client.get_function_app("test-rg", "test-function-rc").await;

    match &result {
        Ok(_) => println!("Function fetched successfully"),
        Err(e) => println!("Azure get function failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_list_functions_in_app() {
    let client = AzureFunctionsClient::new();

    let result = client
        .list_functions_in_app("test-rg", "test-function-rc")
        .await;

    match &result {
        Ok(_) => println!("Functions inside app listed successfully"),
        Err(e) => println!("Azure list functions failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_restart_function_app() {
    let client = AzureFunctionsClient::new();

    let result = client
        .restart_function_app("test-rg", "test-function-rc")
        .await;

    match &result {
        Ok(_) => println!("Function restarted successfully"),
        Err(e) => println!("Azure restart function failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_delete_function_app() {
    let client = AzureFunctionsClient::new();

    let result = client
        .delete_function_app("test-rg", "test-function-rc")
        .await;

    match &result {
        Ok(_) => println!("Function deleted successfully"),
        Err(e) => println!("Azure function deletion failed: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}
