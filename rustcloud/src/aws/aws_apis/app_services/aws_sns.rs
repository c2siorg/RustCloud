#![allow(clippy::result_large_err)]

use aws_sdk_sns::{Client, Error};

pub async fn create_topic(client: &Client, name: &str) -> Result<(), Error> {
    let resp = client.create_topic().name(name).send().await;
    match resp {
        Ok(result) => {
            println!("Topic ARN: {:?}", result.topic_arn());
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete_topic(client: &Client, topic_arn: &str) -> Result<(), Error> {
    let resp = client.delete_topic().topic_arn(topic_arn).send().await;
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

pub async fn list_topics(client: &Client) -> Result<(), Error> {
    let resp = client.list_topics().send().await;
    match resp {
        Ok(result) => {
            for topic in result.topics() {
                println!("Topic ARN: {}", topic.topic_arn().unwrap_or_default());
            }
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn publish(
    client: &Client,
    topic_arn: &str,
    message: &str,
    subject: Option<String>,
) -> Result<(), Error> {
    let resp = client
        .publish()
        .topic_arn(topic_arn)
        .message(message)
        .set_subject(subject)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("Message ID: {:?}", result.message_id());
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn subscribe(
    client: &Client,
    topic_arn: &str,
    protocol: &str,
    endpoint: &str,
) -> Result<(), Error> {
    let resp = client
        .subscribe()
        .topic_arn(topic_arn)
        .protocol(protocol)
        .endpoint(endpoint)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("Subscription ARN: {:?}", result.subscription_arn());
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn unsubscribe(client: &Client, subscription_arn: &str) -> Result<(), Error> {
    let resp = client
        .unsubscribe()
        .subscription_arn(subscription_arn)
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
