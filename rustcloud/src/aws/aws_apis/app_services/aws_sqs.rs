#![allow(clippy::result_large_err)]

use aws_sdk_sqs::{Client, Error};

/// Create a new SQS queue and return its URL.
/// Append `.fifo` to `name` and set `FifoQueue=true` in attributes for FIFO queues.
pub async fn create_queue(client: &Client, name: &str) -> Result<String, Error> {
    let resp = client.create_queue().queue_name(name).send().await;
    match resp {
        Ok(result) => {
            let url = result.queue_url().unwrap_or_default().to_string();
            println!("Queue URL: {}", url);
            Ok(url)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

/// Delete a queue permanently. Messages in the queue are also deleted.
pub async fn delete_queue(client: &Client, queue_url: &str) -> Result<(), Error> {
    let resp = client.delete_queue().queue_url(queue_url).send().await;
    match resp {
        Ok(result) => {
            println!("{:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

/// List queue URLs, optionally filtered by name prefix.
pub async fn list_queues(
    client: &Client,
    prefix: Option<String>,
) -> Result<Vec<String>, Error> {
    let mut builder = client.list_queues();
    if let Some(p) = prefix {
        builder = builder.queue_name_prefix(p);
    }
    let resp = builder.send().await;
    match resp {
        Ok(result) => {
            let urls: Vec<String> = result.queue_urls().iter().map(|u| u.to_string()).collect();
            for url in &urls {
                println!("Queue URL: {}", url);
            }
            Ok(urls)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

/// Resolve a queue name to its full URL.
pub async fn get_queue_url(client: &Client, name: &str) -> Result<String, Error> {
    let resp = client.get_queue_url().queue_name(name).send().await;
    match resp {
        Ok(result) => {
            let url = result.queue_url().unwrap_or_default().to_string();
            println!("Queue URL: {}", url);
            Ok(url)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

/// Send a message to a queue. Returns the message ID assigned by SQS.
/// `delay_seconds` (0–900) defers message visibility; `None` uses the queue default.
pub async fn send_message(
    client: &Client,
    queue_url: &str,
    message_body: &str,
    delay_seconds: Option<i32>,
) -> Result<String, Error> {
    let mut builder = client
        .send_message()
        .queue_url(queue_url)
        .message_body(message_body);
    if let Some(delay) = delay_seconds {
        builder = builder.delay_seconds(delay);
    }
    let resp = builder.send().await;
    match resp {
        Ok(result) => {
            let id = result.message_id().unwrap_or_default().to_string();
            println!("Message ID: {}", id);
            Ok(id)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

/// Receive up to `max_number` messages (1–10) from a queue.
/// Set `wait_time_seconds` > 0 to enable long polling (recommended: 20s).
pub async fn receive_messages(
    client: &Client,
    queue_url: &str,
    max_number: Option<i32>,
    wait_time_seconds: Option<i32>,
) -> Result<Vec<aws_sdk_sqs::types::Message>, Error> {
    let mut builder = client.receive_message().queue_url(queue_url);
    if let Some(max) = max_number {
        builder = builder.max_number_of_messages(max);
    }
    if let Some(wait) = wait_time_seconds {
        builder = builder.wait_time_seconds(wait);
    }
    let resp = builder.send().await;
    match resp {
        Ok(result) => {
            let messages = result.messages().to_vec();
            println!("Received {} message(s)", messages.len());
            Ok(messages)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

/// Delete a message from a queue using its receipt handle.
/// Must be called after processing to prevent redelivery.
pub async fn delete_message(
    client: &Client,
    queue_url: &str,
    receipt_handle: &str,
) -> Result<(), Error> {
    let resp = client
        .delete_message()
        .queue_url(queue_url)
        .receipt_handle(receipt_handle)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("{:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}
