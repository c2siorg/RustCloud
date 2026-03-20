use crate::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;
use crate::traits::llm_provider::LlmProvider;
use crate::types::llm::{LlmRequest, Message, ModelRef};

fn create_provider() -> AzureOpenAIProvider {
    AzureOpenAIProvider::new()
}

#[tokio::test]
async fn test_generate_gpt4o() {
    let provider = create_provider();

    let request = LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello, how are you?".to_string(),
        }],
        max_tokens: Some(100),
        temperature: Some(0.7),
        system_prompt: Some("You are a helpful assistant.".to_string()),
    };

    let result = provider.generate(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(!response.text.is_empty());
    println!("Generated text: {}", response.text);
}

#[tokio::test]
async fn test_generate_gpt35_turbo() {
    let provider = create_provider();

    let request = LlmRequest {
        model: ModelRef::Deployment("gpt-35-turbo".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What is 2+2?".to_string(),
        }],
        max_tokens: Some(50),
        temperature: Some(0.0),
        system_prompt: None,
    };

    let result = provider.generate(request).await;
    assert!(result.is_ok());
    println!("GPT-3.5 response: {:?}", result.unwrap());
}

#[tokio::test]
async fn test_stream_generate() {
    let provider = create_provider();

    let request = LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Tell me a short story in 3 sentences.".to_string(),
        }],
        max_tokens: Some(200),
        temperature: Some(0.8),
        system_prompt: None,
    };

    let result = provider.stream(request).await;
    assert!(result.is_ok());
    println!("Stream initialized successfully");
}

#[tokio::test]
async fn test_embed_texts() {
    let provider = create_provider();

    let texts = vec![
        "Hello world".to_string(),
        "How are you?".to_string(),
        "This is a test.".to_string(),
    ];

    let result = provider.embed(texts).await;
    assert!(result.is_ok());

    let embed_response = result.unwrap();
    assert_eq!(embed_response.embeddings.len(), 3);
    assert!(!embed_response.embeddings[0].is_empty());
    println!("Generated {} embeddings", embed_response.embeddings.len());
}

#[tokio::test]
async fn test_generate_with_tools() {
    use crate::types::llm::ToolDefinition;
    use serde_json::json;

    let provider = create_provider();

    let tools = vec![ToolDefinition {
        name: "get_weather".to_string(),
        description: "Get the current weather for a location".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name"
                }
            },
            "required": ["location"]
        }),
    }];

    let request = LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What is the weather in Seattle?".to_string(),
        }],
        max_tokens: Some(200),
        temperature: Some(0.0),
        system_prompt: None,
    };

    let result = provider.generate_with_tools(request, tools).await;
    assert!(result.is_ok());
    println!("Tool response: {:?}", result.unwrap());
}
