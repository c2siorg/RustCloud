use crate::aws::aws_apis::artificial_intelligence::aws_bedrock::{
    build_inference_config, build_messages, extract_model_id, map_stop_reason,
};
use crate::errors::CloudError;
use crate::types::llm::{LlmRequest, Message, ModelRef};
use aws_sdk_bedrockruntime::types::StopReason;

fn make_request(model: ModelRef, messages: Vec<Message>) -> LlmRequest {
    LlmRequest {
        model,
        messages,
        max_tokens: None,
        temperature: None,
        system_prompt: None,
    }
}

fn user_msg(content: &str) -> Message {
    Message {
        role: "user".to_string(),
        content: content.to_string(),
    }
}

fn assistant_msg(content: &str) -> Message {
    Message {
        role: "assistant".to_string(),
        content: content.to_string(),
    }
}

// ----- extract_model_id -----

#[test]
fn test_extract_model_id_provider_passthrough() {
    let id = "anthropic.claude-3-5-sonnet-20241022-v2:0".to_string();
    let result = extract_model_id(&ModelRef::Provider(id.clone())).unwrap();
    assert_eq!(result, id);
}

#[test]
fn test_extract_model_id_logical_family_and_tier() {
    let model = ModelRef::Logical {
        family: "anthropic.claude-3".to_string(),
        tier: Some("sonnet".to_string()),
    };
    assert_eq!(extract_model_id(&model).unwrap(), "anthropic.claude-3.sonnet");
}

#[test]
fn test_extract_model_id_logical_family_only() {
    let model = ModelRef::Logical {
        family: "amazon.titan-text-express".to_string(),
        tier: None,
    };
    assert_eq!(
        extract_model_id(&model).unwrap(),
        "amazon.titan-text-express"
    );
}

#[test]
fn test_extract_model_id_deployment_is_unsupported() {
    let model = ModelRef::Deployment("my-bedrock-deployment".to_string());
    let err = extract_model_id(&model).unwrap_err();
    assert!(
        matches!(err, CloudError::Unsupported { .. }),
        "expected Unsupported, got {:?}",
        err
    );
}

// ----- map_stop_reason -----

#[test]
fn test_map_stop_reason_end_turn_maps_to_stop() {
    let reason = map_stop_reason(&StopReason::EndTurn);
    assert!(
        matches!(reason, crate::types::llm::FinishReason::Stop),
        "EndTurn should map to Stop"
    );
}

#[test]
fn test_map_stop_reason_max_tokens_maps_to_length() {
    let reason = map_stop_reason(&StopReason::MaxTokens);
    assert!(matches!(reason, crate::types::llm::FinishReason::Length));
}

#[test]
fn test_map_stop_reason_tool_use_maps_to_tool_call() {
    let reason = map_stop_reason(&StopReason::ToolUse);
    assert!(matches!(reason, crate::types::llm::FinishReason::ToolCall));
}

#[test]
fn test_map_stop_reason_unknown_maps_to_other() {
    let reason = map_stop_reason(&StopReason::from("content_filtered"));
    assert!(matches!(reason, crate::types::llm::FinishReason::Other(_)));
}

// ----- build_messages -----

#[test]
fn test_build_messages_single_user_succeeds() {
    let req = make_request(
        ModelRef::Provider("test".to_string()),
        vec![user_msg("Hello, world!")],
    );
    let msgs = build_messages(&req).unwrap();
    assert_eq!(msgs.len(), 1);
}

#[test]
fn test_build_messages_preserves_message_count() {
    let req = make_request(
        ModelRef::Provider("test".to_string()),
        vec![
            user_msg("First message"),
            assistant_msg("First reply"),
            user_msg("Second message"),
        ],
    );
    let msgs = build_messages(&req).unwrap();
    assert_eq!(msgs.len(), 3);
}

#[test]
fn test_build_messages_empty_input_returns_empty_vec() {
    let req = make_request(ModelRef::Provider("test".to_string()), vec![]);
    let msgs = build_messages(&req).unwrap();
    assert!(msgs.is_empty());
}

// ----- build_inference_config -----

#[test]
fn test_build_inference_config_does_not_panic_with_all_fields() {
    let req = LlmRequest {
        model: ModelRef::Provider("test".to_string()),
        messages: vec![],
        max_tokens: Some(1024),
        temperature: Some(0.7),
        system_prompt: None,
    };
    let _ = build_inference_config(&req);
}

#[test]
fn test_build_inference_config_does_not_panic_with_no_fields() {
    let req = make_request(ModelRef::Provider("test".to_string()), vec![]);
    let _ = build_inference_config(&req);
}

// ----- integration -----

#[tokio::test]
#[ignore]
async fn test_generate_live_api() {
    use crate::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
    use crate::traits::llm_provider::LlmProvider;

    let provider = BedrockProvider::new().await;
    let req = LlmRequest {
        model: ModelRef::Provider(
            "anthropic.claude-3-5-haiku-20241022-v1:0".to_string(),
        ),
        messages: vec![user_msg("Reply with the single word OK and nothing else.")],
        max_tokens: Some(10),
        temperature: Some(0.0),
        system_prompt: None,
    };
    let result = provider.generate(req).await.unwrap();
    assert!(!result.text.is_empty());
}
