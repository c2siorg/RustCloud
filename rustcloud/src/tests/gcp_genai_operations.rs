#[allow(unused_imports)]
use crate::gcp::gcp_apis::artificial_intelligence::gcp_genai::{Backend, GoogleGenAiAdapter};
#[allow(unused_imports)]
use crate::traits::llm_provider::LlmProvider;
#[allow(unused_imports)]
use crate::types::llm::{LlmRequest, Message, ModelRef, ToolDefinition};
#[allow(unused_imports)]
use futures::StreamExt;

fn create_adapter() -> GoogleGenAiAdapter {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set to run GenAI integration tests");
    GoogleGenAiAdapter::new(
        Backend::GeminiApi { api_key },
        "gemini-2.5-flash".to_string(),
    )
}

fn simple_request(prompt: &str) -> LlmRequest {
    LlmRequest {
        model: ModelRef::Provider("gemini-2.5-flash".to_string()),
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
async fn test_genai_generate() {
    let adapter = create_adapter();
    let req = simple_request("Say hello in one sentence.");

    let result = adapter.generate(req).await;
    if let Err(ref e) = result {
        println!("ERROR: {}", e);
    }
    assert!(result.is_ok());
    println!("generate response: {:?}", result.unwrap().text);
}

#[tokio::test]
async fn test_genai_stream() {
    let adapter = create_adapter();
    let req = simple_request("Count from 1 to 5.");

    let result = adapter.stream(req).await;
    assert!(result.is_ok());

    let mut stream = result.unwrap();
    while let Some(event) = stream.next().await {
        println!("stream event: {:?}", event);
    }
}

#[tokio::test]
async fn test_genai_embed() {
    let adapter = create_adapter();
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
async fn test_genai_generate_with_tools() {
    let adapter = create_adapter();
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
    assert!(result.is_ok());
    println!("tool response: {:?}", result.unwrap());
}
