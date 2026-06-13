use crate::aws::aws_apis::artificial_intelligence::aws_bedrock::{
    build_inference_config, build_messages, build_tool_spec, classify_service_error,
    document_to_json, extract_model_id, json_to_document, map_stop_reason, map_stream_event,
    parse_embed_response,
};
use crate::errors::CloudError;
use crate::types::llm::{LlmRequest, Message, ModelRef, ToolDefinition};
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

// ----- parse_embed_response -----

#[test]
fn test_embed_single_text_returns_one_vector() {
    let json = serde_json::json!({"embedding": [0.1f32, 0.2f32, 0.3f32], "inputTextTokenCount": 3});
    let vector = parse_embed_response(&json).unwrap();
    assert_eq!(vector.len(), 3);
}

#[test]
fn test_embed_multiple_texts_returns_multiple_vectors() {
    let responses = vec![
        serde_json::json!({"embedding": [0.1f32, 0.2f32]}),
        serde_json::json!({"embedding": [0.3f32, 0.4f32]}),
    ];
    let vectors: Vec<Vec<f32>> = responses
        .iter()
        .map(|j| parse_embed_response(j).unwrap())
        .collect();
    assert_eq!(vectors.len(), 2);
    assert_eq!(vectors[0].len(), 2);
    assert_eq!(vectors[1].len(), 2);
}

#[tokio::test]
async fn test_embed_empty_input_returns_empty_vec() {
    use crate::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
    use crate::traits::llm_provider::LlmProvider;

    let config = aws_sdk_bedrockruntime::Config::builder()
        .behavior_version(aws_sdk_bedrockruntime::config::BehaviorVersion::latest())
        .build();
    let provider = BedrockProvider::with_client(aws_sdk_bedrockruntime::Client::from_conf(config));
    let result = provider.embed(vec![]).await.unwrap();
    assert!(result.embeddings.is_empty());
}

// ----- map_stream_event -----

#[test]
fn test_map_stream_event_text_delta_maps_to_delta_text() {
    use aws_sdk_bedrockruntime::types::{
        ContentBlockDelta, ContentBlockDeltaEvent, ConverseStreamOutput,
    };

    let event = ConverseStreamOutput::ContentBlockDelta(
        ContentBlockDeltaEvent::builder()
            .content_block_index(0)
            .delta(ContentBlockDelta::Text("hello".to_string()))
            .build()
            .unwrap(),
    );
    let result = map_stream_event(&event);
    assert!(
        matches!(result, Some(crate::types::llm::LlmStreamEvent::DeltaText(ref t)) if t == "hello"),
        "expected DeltaText(\"hello\"), got {:?}",
        result
    );
}

#[test]
fn test_map_stream_event_message_stop_maps_to_done() {
    use aws_sdk_bedrockruntime::types::{ConverseStreamOutput, MessageStopEvent};

    let event = ConverseStreamOutput::MessageStop(
        MessageStopEvent::builder()
            .stop_reason(StopReason::EndTurn)
            .build()
            .unwrap(),
    );
    let result = map_stream_event(&event);
    assert!(
        matches!(
            result,
            Some(crate::types::llm::LlmStreamEvent::Done(
                crate::types::llm::FinishReason::Stop
            ))
        ),
        "expected Done(Stop), got {:?}",
        result
    );
}

#[test]
fn test_map_stream_event_metadata_maps_to_usage() {
    use aws_sdk_bedrockruntime::types::{
        ConverseStreamMetadataEvent, ConverseStreamOutput, TokenUsage,
    };

    let usage = TokenUsage::builder()
        .input_tokens(10)
        .output_tokens(20)
        .total_tokens(30)
        .build()
        .unwrap();
    let event = ConverseStreamOutput::Metadata(
        ConverseStreamMetadataEvent::builder()
            .usage(usage)
            .build(),
    );
    let result = map_stream_event(&event);
    assert!(
        matches!(
            result,
            Some(crate::types::llm::LlmStreamEvent::Usage(ref u))
                if u.prompt_tokens == 10 && u.completion_tokens == 20
        ),
        "expected Usage(10, 20), got {:?}",
        result
    );
}

#[test]
fn test_map_stream_event_non_content_event_returns_none() {
    use aws_sdk_bedrockruntime::types::{ContentBlockStopEvent, ConverseStreamOutput};

    let event = ConverseStreamOutput::ContentBlockStop(
        ContentBlockStopEvent::builder()
            .content_block_index(0)
            .build()
            .unwrap(),
    );
    assert!(
        map_stream_event(&event).is_none(),
        "ContentBlockStop should return None"
    );
}

// ----- json_to_document / document_to_json -----

#[test]
fn test_json_to_document_preserves_string() {
    let val = serde_json::json!("hello");
    assert_eq!(document_to_json(json_to_document(val.clone())), val);
}

#[test]
fn test_json_to_document_preserves_object() {
    let val = serde_json::json!({"type": "object", "count": 3, "active": true});
    assert_eq!(document_to_json(json_to_document(val.clone())), val);
}

#[test]
fn test_document_round_trip_nested() {
    let val = serde_json::json!({"nested": {"arr": [1, 2, 3], "flag": false}});
    assert_eq!(document_to_json(json_to_document(val.clone())), val);
}

// ----- build_tool_spec -----

#[test]
fn test_build_tool_spec_sets_name_and_description() {
    let tool = ToolDefinition {
        name: "weather".to_string(),
        description: "Get current weather".to_string(),
        parameters: serde_json::json!({"type": "object", "properties": {}}),
    };
    let spec = build_tool_spec(&tool).unwrap();
    assert_eq!(spec.name(), "weather");
    assert_eq!(spec.description(), Some("Get current weather"));
}

// ----- classify_service_error -----

#[test]
fn test_classify_service_error_throttling_by_status_returns_rate_limit() {
    let err = classify_service_error(429, "", "rate limited".to_string());
    assert!(matches!(err, CloudError::RateLimit { .. }));
}

#[test]
fn test_classify_service_error_throttling_by_code_returns_rate_limit() {
    let err = classify_service_error(200, "ThrottlingException", "throttled".to_string());
    assert!(matches!(err, CloudError::RateLimit { .. }));
}

#[test]
fn test_classify_service_error_access_denied_by_status_returns_auth() {
    let err = classify_service_error(403, "", "forbidden".to_string());
    assert!(matches!(err, CloudError::Auth { .. }));
}

#[test]
fn test_classify_service_error_access_denied_by_code_returns_auth() {
    let err = classify_service_error(200, "AccessDeniedException", "denied".to_string());
    assert!(matches!(err, CloudError::Auth { .. }));
}

#[test]
fn test_classify_service_error_validation_returns_provider_not_retryable() {
    let err = classify_service_error(400, "ValidationException", "bad request".to_string());
    assert!(
        matches!(err, CloudError::Provider { retryable: false, .. }),
        "expected Provider {{ retryable: false }}, got {:?}",
        err
    );
}

#[test]
fn test_classify_service_error_service_unavailable_returns_provider_retryable() {
    let err = classify_service_error(503, "", "unavailable".to_string());
    assert!(
        matches!(err, CloudError::Provider { retryable: true, .. }),
        "expected Provider {{ retryable: true }}, got {:?}",
        err
    );
}

#[test]
fn test_classify_service_error_unknown_5xx_is_retryable() {
    let err = classify_service_error(502, "", "bad gateway".to_string());
    assert!(
        matches!(err, CloudError::Provider { http_status: 502, retryable: true, .. }),
        "expected Provider {{ http_status: 502, retryable: true }}, got {:?}",
        err
    );
}

#[test]
fn test_classify_service_error_unknown_4xx_is_not_retryable() {
    let err = classify_service_error(422, "", "unprocessable".to_string());
    assert!(
        matches!(err, CloudError::Provider { http_status: 422, retryable: false, .. }),
        "expected Provider {{ http_status: 422, retryable: false }}, got {:?}",
        err
    );
}

#[test]
fn test_classify_service_error_unauthorized_by_code_returns_auth() {
    let err = classify_service_error(200, "UnauthorizedException", "not authorized".to_string());
    assert!(matches!(err, CloudError::Auth { .. }));
}

#[test]
fn test_classify_service_error_model_timeout_returns_provider_retryable() {
    let err = classify_service_error(408, "ModelTimeoutException", "timed out".to_string());
    assert!(
        matches!(err, CloudError::Provider { http_status: 408, retryable: true, .. }),
        "expected Provider {{ http_status: 408, retryable: true }}, got {:?}",
        err
    );
}

#[tokio::test]
async fn test_generate_with_tools_empty_tools_returns_unsupported() {
    use crate::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
    use crate::traits::llm_provider::LlmProvider;

    let config = aws_sdk_bedrockruntime::Config::builder()
        .behavior_version(aws_sdk_bedrockruntime::config::BehaviorVersion::latest())
        .build();
    let provider = BedrockProvider::with_client(aws_sdk_bedrockruntime::Client::from_conf(config));
    let req = make_request(
        ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
        vec![user_msg("test")],
    );
    let err = provider.generate_with_tools(req, vec![]).await.unwrap_err();
    assert!(
        matches!(err, CloudError::Unsupported { .. }),
        "expected Unsupported, got {:?}",
        err
    );
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

#[tokio::test]
#[ignore]
async fn test_embed_live_api() {
    use crate::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
    use crate::traits::llm_provider::LlmProvider;

    let provider = BedrockProvider::new().await;
    let result = provider
        .embed(vec!["Rust programming language".to_string()])
        .await
        .unwrap();
    assert_eq!(result.embeddings.len(), 1);
    assert!(!result.embeddings[0].is_empty());
}

#[tokio::test]
#[ignore]
async fn test_stream_live_api() {
    use crate::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
    use crate::traits::llm_provider::LlmProvider;
    use crate::types::llm::LlmStreamEvent;
    use futures::StreamExt;

    let provider = BedrockProvider::new().await;
    let req = LlmRequest {
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
        messages: vec![user_msg("Reply with the single word OK and nothing else.")],
        max_tokens: Some(10),
        temperature: Some(0.0),
        system_prompt: None,
    };
    let mut stream = provider.stream(req).await.unwrap();
    let mut got_text = false;
    while let Some(event) = stream.next().await {
        if matches!(event, LlmStreamEvent::DeltaText(_)) {
            got_text = true;
        }
    }
    assert!(got_text);
}

#[tokio::test]
#[ignore]
async fn test_generate_with_tools_live_api() {
    use crate::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
    use crate::traits::llm_provider::LlmProvider;
    use crate::types::llm::ToolCallResponse;

    let provider = BedrockProvider::new().await;
    let req = LlmRequest {
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
        messages: vec![user_msg("What is the weather in London?")],
        max_tokens: Some(256),
        temperature: Some(0.0),
        system_prompt: None,
    };
    let tools = vec![ToolDefinition {
        name: "get_weather".to_string(),
        description: "Get the current weather for a location".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "location": {"type": "string", "description": "City name"}
            },
            "required": ["location"]
        }),
    }];
    let result = provider.generate_with_tools(req, tools).await.unwrap();
    assert!(matches!(result, ToolCallResponse::ToolCall { .. }));
}
