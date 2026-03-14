use crate::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAI;
use crate::traits::llm_provider::LlmProvider;
use crate::types::llm::{LlmRequest, Message, ModelRef};

#[tokio::test]
async fn test_generate_text() {
    let vertex_ai = VertexAI::new("your-project-id", "us-central1");

    let request = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello, how are you?".to_string(),
        }],
        max_tokens: Some(100),
        temperature: Some(0.7),
        system_prompt: Some("You are a helpful assistant.".to_string()),
    };

    let result = vertex_ai.generate(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(!response.text.is_empty());
    println!("Generated text: {}", response.text);
}

#[tokio::test]
async fn test_generate_with_system_prompt() {
    let vertex_ai = VertexAI::new("your-project-id", "us-central1");

    let request = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-pro".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What is 2+2?".to_string(),
        }],
        max_tokens: Some(50),
        temperature: Some(0.0),
        system_prompt: Some("Answer concisely with just the number.".to_string()),
    };

    let result = vertex_ai.generate(request).await;
    assert!(result.is_ok());
    println!("Math response: {:?}", result.unwrap());
}

#[tokio::test]
async fn test_embed_texts() {
    let vertex_ai = VertexAI::new("your-project-id", "us-central1");

    let texts = vec![
        "Hello world".to_string(),
        "How are you?".to_string(),
        "This is a test.".to_string(),
    ];

    let result = vertex_ai.embed(texts).await;
    assert!(result.is_ok());

    let embed_response = result.unwrap();
    assert_eq!(embed_response.embeddings.len(), 3);
    assert!(!embed_response.embeddings[0].is_empty());
    println!("Generated {} embeddings", embed_response.embeddings.len());
}

#[tokio::test]
async fn test_stream_generate() {
    let vertex_ai = VertexAI::new("your-project-id", "us-central1");

    let request = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Tell me a short story.".to_string(),
        }],
        max_tokens: Some(200),
        temperature: Some(0.8),
        system_prompt: None,
    };

    let result = vertex_ai.stream(request).await;
    assert!(result.is_ok());
    println!("Stream initialized successfully");
}

#[tokio::test]
async fn test_generate_with_tools() {
    use crate::types::llm::ToolDefinition;
    use serde_json::json;

    let vertex_ai = VertexAI::new("your-project-id", "us-central1");

    let tools = vec![ToolDefinition {
        name: "get_weather".to_string(),
        description: "Get the current weather for a location".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The location to get weather for"
                }
            },
            "required": ["location"]
        }),
    }];

    let request = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-pro".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What is the weather in New York?".to_string(),
        }],
        max_tokens: Some(100),
        temperature: Some(0.0),
        system_prompt: None,
    };

    let result = vertex_ai.generate_with_tools(request, tools).await;
    assert!(result.is_ok());
    println!("Generate with tools response: {:?}", result.unwrap());
}
