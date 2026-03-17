#![allow(clippy::result_large_err)]

use aws_sdk_lambda::{
    primitives::Blob,
    types::{FunctionCode, Runtime},
    Client, Error,
};

pub async fn create_function(
    client: &Client,
    function_name: &str,
    runtime: Runtime,
    role_arn: &str,
    handler: &str,
    zip_bytes: Vec<u8>,
) -> Result<String, Error> {
    let code = FunctionCode::builder()
        .zip_file(Blob::new(zip_bytes))
        .build();

    let resp = client
        .create_function()
        .function_name(function_name)
        .runtime(runtime)
        .role(role_arn)
        .handler(handler)
        .code(code)
        .send()
        .await;

    match resp {
        Ok(result) => {
            let arn = result.function_arn().unwrap_or_default().to_string();
            println!("Created Lambda function ARN: {}", arn);
            Ok(arn)
        }
        Err(e) => {
            println!("Error creating Lambda function: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete_function(client: &Client, function_name: &str) -> Result<(), Error> {
    let resp = client
        .delete_function()
        .function_name(function_name)
        .send()
        .await;

    match resp {
        Ok(_) => {
            println!("Deleted Lambda function: {}", function_name);
            Ok(())
        }
        Err(e) => {
            println!("Error deleting Lambda function: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn list_functions(client: &Client) -> Result<Vec<String>, Error> {
    let resp = client.list_functions().send().await;

    match resp {
        Ok(result) => {
            let names: Vec<String> = result
                .functions()
                .iter()
                .map(|f| f.function_name().unwrap_or_default().to_string())
                .collect();
            println!("Lambda functions: {:?}", names);
            Ok(names)
        }
        Err(e) => {
            println!("Error listing Lambda functions: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn get_function(client: &Client, function_name: &str) -> Result<(), Error> {
    let resp = client
        .get_function()
        .function_name(function_name)
        .send()
        .await;

    match resp {
        Ok(result) => {
            if let Some(config) = result.configuration() {
                println!("Function name: {:?}", config.function_name());
                println!("Runtime: {:?}", config.runtime());
                println!("Handler: {:?}", config.handler());
                println!("State: {:?}", config.state());
                println!("Last modified: {:?}", config.last_modified());
            }
            Ok(())
        }
        Err(e) => {
            println!("Error getting Lambda function: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn invoke_function(
    client: &Client,
    function_name: &str,
    payload: Option<Vec<u8>>,
) -> Result<Option<Vec<u8>>, Error> {
    let mut req = client.invoke().function_name(function_name);

    if let Some(p) = payload {
        req = req.payload(Blob::new(p));
    }

    let resp = req.send().await;

    match resp {
        Ok(result) => {
            println!("Invocation status code: {:?}", result.status_code());
            if let Some(err) = result.function_error() {
                println!("Function error: {}", err);
            }
            let body = result.payload().map(|b| b.as_ref().to_vec());
            Ok(body)
        }
        Err(e) => {
            println!("Error invoking Lambda function: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn update_function_code(
    client: &Client,
    function_name: &str,
    zip_bytes: Vec<u8>,
) -> Result<(), Error> {
    let resp = client
        .update_function_code()
        .function_name(function_name)
        .zip_file(Blob::new(zip_bytes))
        .send()
        .await;

    match resp {
        Ok(result) => {
            println!("Updated function code ARN: {:?}", result.function_arn());
            Ok(())
        }
        Err(e) => {
            println!("Error updating Lambda function code: {:?}", e);
            Err(e.into())
        }
    }
}
