use crate::types::llm::{LlmRequest, Message, MessageRole, ModelRef, FinishReason, UsageStats, LlmResponse};

#[test]
fn test_llm_request_builder() {
    let request = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash".to_string()),
        messages: vec![
            Message { role: MessageRole::User, content: "Hello".to_string() },
            Message { role: MessageRole::Model, content: "Hi there".to_string() },
        ],
        max_tokens: Some(100),
        temperature: Some(0.9),
        system_prompt: Some("You are a helpful assistant".to_string()),
    };
    
    assert_eq!(request.messages.len(), 2);
    assert_eq!(request.max_tokens, Some(100));
    assert!(request.system_prompt.is_some());
}

#[test]
fn test_message_role_validation() {
    let user_msg = Message { role: MessageRole::User, content: "Test".to_string() };
    let model_msg = Message { role: MessageRole::Model, content: "Response".to_string() };
    let system_msg = Message { role: MessageRole::System, content: "System prompt".to_string() };
    let tool_msg = Message { role: MessageRole::Tool, content: "Tool result".to_string() };
    
    match user_msg.role { MessageRole::User => (), _ => panic!() }
    match model_msg.role { MessageRole::Model => (), _ => panic!() }
    match system_msg.role { MessageRole::System => (), _ => panic!() }
    match tool_msg.role { MessageRole::Tool => (), _ => panic!() }
}

#[test]
fn test_model_ref_variants() {
    let provider_ref = ModelRef::Provider("gemini-1.5-pro".to_string());
    let logical_ref = ModelRef::Logical { family: "gemini".to_string(), tier: Some("001".to_string()) };
    let deployment_ref = ModelRef::Deployment("my-deployment".to_string());
    
    match provider_ref {
        ModelRef::Provider(id) => assert_eq!(id, "gemini-1.5-pro"),
        _ => panic!("Expected Provider variant"),
    }
    
    match logical_ref {
        ModelRef::Logical { family, tier } => {
            assert_eq!(family, "gemini");
            assert_eq!(tier, Some("001".to_string()));
        }
        _ => panic!("Expected Logical variant"),
    }
    
    match deployment_ref {
        ModelRef::Deployment(id) => assert_eq!(id, "my-deployment"),
        _ => panic!("Expected Deployment variant"),
    }
}

#[test]
fn test_finish_reason_variants() {
    let stop = FinishReason::Stop;
    let length = FinishReason::Length;
    let tool_call = FinishReason::ToolCall;
    let other = FinishReason::Other("CUSTOM".to_string());
    
    match stop { FinishReason::Stop => (), _ => panic!() }
    match length { FinishReason::Length => (), _ => panic!() }
    match tool_call { FinishReason::ToolCall => (), _ => panic!() }
    match other { FinishReason::Other(ref s) if s == "CUSTOM" => (), _ => panic!() }
}

#[test]
fn test_llm_response_structure() {
    let response = LlmResponse {
        text: "Hello, world!".to_string(),
        finish_reason: FinishReason::Stop,
        usage: Some(UsageStats { prompt_tokens: 10, completion_tokens: 5 }),
    };
    
    assert_eq!(response.text, "Hello, world!");
    assert_eq!(response.usage.as_ref().map(|u| u.prompt_tokens), Some(10));
    assert_eq!(response.usage.as_ref().map(|u| u.completion_tokens), Some(5));
}

#[test]
fn test_usage_stats_optional() {
    let with_usage = LlmResponse {
        text: "test".to_string(),
        finish_reason: FinishReason::Stop,
        usage: Some(UsageStats { prompt_tokens: 1, completion_tokens: 2 }),
    };
    
    let without_usage = LlmResponse {
        text: "test".to_string(),
        finish_reason: FinishReason::Stop,
        usage: None,
    };
    
    assert!(with_usage.usage.is_some());
    assert!(without_usage.usage.is_none());
}
