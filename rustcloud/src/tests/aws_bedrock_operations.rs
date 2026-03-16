use crate::aws::aws_apis::ai::bedrock::AwsBedrockGenAI;
use crate::types::llm::{LlmRequest, Message, ModelRef};

#[tokio::test]
async fn test_bedrock_compilation() {
    let _client = AwsBedrockGenAI::new().await;
    let _req = LlmRequest {
        model: ModelRef::Provider("amazon.titan-text-lite-v1".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello from RustCloud".to_string(),
        }],
        max_tokens: Some(50),
        temperature: Some(0.7),
        system_prompt: None,
    };
    
    assert!(true);
}
