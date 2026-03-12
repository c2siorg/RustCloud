use crate::azure::azure_apis::compute::azure_vm::AzureVMClient;

#[tokio::test]
async fn test_create_vm() {
    let client = AzureVMClient::new();

    let nic_id =
        "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/test-rg/providers/Microsoft.Network/networkInterfaces/test-nic";

    let result = client
        .create_vm("test-rg", "test-vm", "eastasia", nic_id)
        .await;

    match &result {
        Ok(_) => println!("VM created successfully"),
        Err(e) => println!("Azure VM creation failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}

#[tokio::test]
async fn test_list_vms() {
    let client = AzureVMClient::new();
    let resource_group = "test-rg";

    let result = client.list_vms(resource_group).await;

    match &result {
        Ok(vms) => println!("List VMs result: {:?}", vms),
        Err(e) => println!("Azure list VMs failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_start_vm() {
    let client = AzureVMClient::new();

    let result = client.start_vm("test-rg", "test-vm").await;

    match &result {
        Ok(_) => println!("VM started successfully"),
        Err(e) => println!("Azure VM start failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_stop_vm() {
    let client = AzureVMClient::new();

    let result = client.stop_vm("test-rg", "test-vm").await;

    match &result {
        Ok(_) => println!("VM stopped successfully"),
        Err(e) => println!("Azure VM stop failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_get_vm() {
    let client = AzureVMClient::new();

    let result = client.get_vm("test-rg", "test-vm").await;

    match &result {
        Ok(vm) => println!("Get VM result: {:?}", vm),
        Err(e) => println!("Azure get VM failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_restart_vm() {
    let client = AzureVMClient::new();

    let result = client.restart_vm("test-rg", "test-vm").await;

    match &result {
        Ok(_) => println!("VM restarted successfully"),
        Err(e) => println!("Azure VM restart failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_delete_vm() {
    let client = AzureVMClient::new();

    let result = client.delete_vm("test-rg", "test-vm").await;

    match &result {
        Ok(_) => println!("VM deleted successfully"),
        Err(e) => println!("Azure VM deletion failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}

#[tokio::test]
async fn test_vm_instance_view() {
    let client = AzureVMClient::new();

    let result = client.vm_instance_view("test-rg", "test-vm").await;

    match &result {
        Ok(view) => println!("VM Instance view result: {:?}", view),
        Err(e) => println!("Azure VM Instance view failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}
