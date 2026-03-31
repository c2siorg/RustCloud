use crate::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockAdapter;
use crate::traits::llm_provider::LlmProvider;
use crate::types::llm::{LlmRequest, Message, ModelRef, ToolDefinition};
use futures::StreamExt;

// Requires AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_DEFAULT_REGION
const MODEL: &str = "amazon.nova-micro-v1:0";

async fn create_adapter() -> BedrockAdapter {
    BedrockAdapter::new().await
}

fn simple_request(prompt: &str) -> LlmRequest {
    LlmRequest {
        model: ModelRef::Provider(MODEL.to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        max_tokens: Some(256),
        temperature: Some(0.7),
        system_prompt: None,
    }
}

#[tokio::test]
async fn test_bedrock_generate() {
    let adapter = create_adapter().await;
    let req = simple_request("Say hello in one sentence.");

    let result = adapter.generate(req).await;
    if let Err(ref e) = result {
        println!("ERROR: {}", e);
    }
    assert!(result.is_ok());
    println!("generate response: {:?}", result.unwrap().text);
}

#[tokio::test]
async fn test_bedrock_stream() {
    let adapter = create_adapter().await;
    let req = simple_request("Count from 1 to 5.");

    let result = adapter.stream(req).await;
    assert!(result.is_ok());

    let mut stream = result.unwrap();
    while let Some(event) = stream.next().await {
        println!("stream event: {:?}", event);
    }
}

#[tokio::test]
async fn test_bedrock_embed() {
    let adapter = create_adapter().await;
    let texts = vec!["Hello world".to_string(), "Rust is fast".to_string()];

    let result = adapter.embed(texts).await;
    if let Err(ref e) = result {
        println!("EMBED ERROR: {}", e);
    }
    assert!(result.is_ok());
    let embeddings = result.unwrap().embeddings;
    assert_eq!(embeddings.len(), 2);
    println!("embedding dimensions: {}", embeddings[0].len());
}

#[tokio::test]
async fn test_bedrock_generate_with_tools() {
    let adapter = create_adapter().await;
    let req = simple_request("What is the weather in Mumbai?");

    let tools = vec![ToolDefinition {
        name: "get_weather".to_string(),
        description: "Get current weather for a city".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "city": { "type": "string", "description": "City name" }
            },
            "required": ["city"]
        }),
    }];

    let result = adapter.generate_with_tools(req, tools).await;
    if let Err(ref e) = result {
        println!("TOOLS ERROR: {}", e);
    }
    assert!(result.is_ok());
    println!("tool response: {:?}", result.unwrap());
}
