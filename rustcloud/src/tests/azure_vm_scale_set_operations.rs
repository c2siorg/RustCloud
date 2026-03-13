use crate::azure::azure_apis::compute::azure_vm_scale_set::AzureVMScaleSetClient;

#[tokio::test]
async fn test_create_vmss() {
    let client = AzureVMScaleSetClient::new();

    let subnet_id = "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/test-rg/providers/Microsoft.Network/virtualNetworks/test-vnet/subnets/default";

    let result = client
        .create_vmss("test-rg", "test-vmss", "eastasia", subnet_id)
        .await;

    match &result {
        Ok(_) => println!("VMSS created successfully"),
        Err(e) => println!("Azure VMSS creation failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}

#[tokio::test]
async fn test_list_vmss() {
    let client = AzureVMScaleSetClient::new();

    let result = client.list_vmss("test-rg").await;

    match &result {
        Ok(_) => println!("VMSS list successful"),
        Err(e) => println!("Azure VMSS list failed: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}

#[tokio::test]
async fn test_get_vmss() {
    let client = AzureVMScaleSetClient::new();

    let result = client.get_vmss("test-rg", "test-vmss").await;

    match &result {
        Ok(_) => println!("VMSS fetched successfully"),
        Err(e) => println!("Azure VMSS get failed: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}

#[tokio::test]
async fn test_delete_vmss() {
    let client = AzureVMScaleSetClient::new();

    let result = client.delete_vmss("test-rg", "test-vmss").await;

    match &result {
        Ok(_) => println!("VMSS deleted successfully"),
        Err(e) => println!("Azure VMSS deletion failed: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}
