use std::sync::Arc;

use crate::errors::CloudError;
use crate::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::{
    VertexAiProvider,
    build_embed_request,
    build_vertex_request,
    extract_model_id,
    map_finish_reason,
    map_vertex_http_error,
    parse_embed_response,
    parse_sse_line,
    parse_vertex_response,
    sse_chunk_to_events,
    vertex_endpoint,
};
use crate::gcp::gcp_apis::auth::gcp_auth::MockTokenProvider;
use crate::traits::llm_provider::LlmProvider;
use crate::traits::token_provider::TokenProvider;
use crate::types::llm::{FinishReason, LlmRequest, LlmStreamEvent, Message, ModelRef};

fn no_creds_provider() -> VertexAiProvider {
    VertexAiProvider::with_http_client(
        reqwest::Client::new(),
        "test-project",
        "us-central1",
        Arc::new(MockTokenProvider::new("fake-token")),
    )
}

fn make_request(model: ModelRef, messages: Vec<Message>) -> LlmRequest {
    LlmRequest {
        model,
        messages,
        max_tokens: None,
        temperature: None,
        system_prompt: None,
    }
}

// --- extract_model_id ---

#[test]
fn test_extract_model_id_provider_passthrough() {
    let result = extract_model_id(&ModelRef::Provider("gemini-1.5-pro-001".to_string()));
    assert_eq!(result.unwrap(), "gemini-1.5-pro-001");
}

#[test]
fn test_extract_model_id_logical_with_tier() {
    let result = extract_model_id(&ModelRef::Logical {
        family: "gemini".to_string(),
        tier: Some("1.5-flash".to_string()),
    });
    assert_eq!(result.unwrap(), "gemini-1.5-flash");
}

#[test]
fn test_extract_model_id_logical_without_tier() {
    let result = extract_model_id(&ModelRef::Logical {
        family: "gemini".to_string(),
        tier: None,
    });
    assert_eq!(result.unwrap(), "gemini");
}

#[test]
fn test_extract_model_id_deployment_is_unsupported() {
    let err = extract_model_id(&ModelRef::Deployment("my-model".to_string())).unwrap_err();
    assert!(
        matches!(err, CloudError::Unsupported { .. }),
        "expected Unsupported, got {:?}",
        err
    );
}

// --- vertex_endpoint ---

#[test]
fn test_vertex_endpoint_format() {
    let url = vertex_endpoint("my-project", "us-central1", "gemini-1.5-pro", "generateContent");
    assert_eq!(
        url,
        "https://us-central1-aiplatform.googleapis.com/v1/projects/my-project\
/locations/us-central1/publishers/google/models/gemini-1.5-pro:generateContent"
    );
}

// --- build_vertex_request ---

#[test]
fn test_build_vertex_request_user_role() {
    let req = make_request(
        ModelRef::Provider("gemini-1.5-flash".to_string()),
        vec![Message { role: "user".to_string(), content: "hello".to_string() }],
    );
    let body = build_vertex_request(&req).unwrap();
    assert_eq!(body["contents"][0]["role"], "user");
    assert_eq!(body["contents"][0]["parts"][0]["text"], "hello");
}

#[test]
fn test_build_vertex_request_assistant_maps_to_model() {
    let req = make_request(
        ModelRef::Provider("gemini-1.5-flash".to_string()),
        vec![Message { role: "assistant".to_string(), content: "ok".to_string() }],
    );
    let body = build_vertex_request(&req).unwrap();
    assert_eq!(body["contents"][0]["role"], "model");
}

#[test]
fn test_build_vertex_request_unknown_role_returns_error() {
    let req = make_request(
        ModelRef::Provider("gemini-1.5-flash".to_string()),
        vec![Message { role: "system".to_string(), content: "be helpful".to_string() }],
    );
    let err = build_vertex_request(&req).unwrap_err();
    assert!(
        matches!(err, CloudError::Provider { .. }),
        "expected Provider error for unknown role, got {:?}",
        err
    );
}

#[test]
fn test_build_vertex_request_includes_system_prompt() {
    let mut req = make_request(ModelRef::Provider("g".to_string()), vec![]);
    req.system_prompt = Some("You are a helpful assistant.".to_string());
    let body = build_vertex_request(&req).unwrap();
    assert_eq!(
        body["systemInstruction"]["parts"][0]["text"],
        "You are a helpful assistant."
    );
}

#[test]
fn test_build_vertex_request_includes_gen_config_when_set() {
    let mut req = make_request(ModelRef::Provider("g".to_string()), vec![]);
    req.max_tokens = Some(512);
    req.temperature = Some(0.5);
    let body = build_vertex_request(&req).unwrap();
    assert_eq!(body["generationConfig"]["maxOutputTokens"], 512);
    assert!((body["generationConfig"]["temperature"].as_f64().unwrap() - 0.5).abs() < 1e-6);
}

#[test]
fn test_build_vertex_request_omits_gen_config_when_all_none() {
    let req = make_request(ModelRef::Provider("g".to_string()), vec![]);
    let body = build_vertex_request(&req).unwrap();
    assert!(body["generationConfig"].is_null());
}

// --- map_finish_reason ---

#[test]
fn test_map_finish_reason_stop() {
    assert!(matches!(map_finish_reason("STOP"), FinishReason::Stop));
}

#[test]
fn test_map_finish_reason_max_tokens() {
    assert!(matches!(map_finish_reason("MAX_TOKENS"), FinishReason::Length));
}

#[test]
fn test_map_finish_reason_tool_calls() {
    assert!(matches!(map_finish_reason("TOOL_CALLS"), FinishReason::ToolCall));
}

#[test]
fn test_map_finish_reason_function_call() {
    assert!(matches!(map_finish_reason("FUNCTION_CALL"), FinishReason::ToolCall));
}

#[test]
fn test_map_finish_reason_unknown() {
    let r = map_finish_reason("RECITATION");
    assert!(matches!(r, FinishReason::Other(s) if s == "RECITATION"));
}

// --- parse_vertex_response ---

#[test]
fn test_parse_vertex_response_extracts_text_and_finish_reason() {
    let json = serde_json::json!({
        "candidates": [{
            "content": { "parts": [{ "text": "Hello!" }], "role": "model" },
            "finishReason": "STOP"
        }],
        "usageMetadata": { "promptTokenCount": 5, "candidatesTokenCount": 2 }
    });
    let resp = parse_vertex_response(&json).unwrap();
    assert_eq!(resp.text, "Hello!");
    assert!(matches!(resp.finish_reason, FinishReason::Stop));
}

#[test]
fn test_parse_vertex_response_extracts_usage() {
    let json = serde_json::json!({
        "candidates": [{
            "content": { "parts": [{ "text": "ok" }] },
            "finishReason": "STOP"
        }],
        "usageMetadata": { "promptTokenCount": 10, "candidatesTokenCount": 20 }
    });
    let resp = parse_vertex_response(&json).unwrap();
    let usage = resp.usage.unwrap();
    assert_eq!(usage.prompt_tokens, 10);
    assert_eq!(usage.completion_tokens, 20);
}

#[test]
fn test_parse_vertex_response_missing_usage_returns_none() {
    let json = serde_json::json!({
        "candidates": [{
            "content": { "parts": [{ "text": "ok" }] },
            "finishReason": "STOP"
        }]
    });
    let resp = parse_vertex_response(&json).unwrap();
    assert!(resp.usage.is_none());
}

#[test]
fn test_parse_vertex_response_no_candidates_returns_error() {
    let json = serde_json::json!({ "candidates": [] });
    let err = parse_vertex_response(&json).unwrap_err();
    assert!(
        matches!(err, CloudError::Provider { .. }),
        "expected Provider error, got {:?}",
        err
    );
}

// --- map_vertex_http_error ---

#[test]
fn test_map_vertex_http_error_401_is_auth() {
    assert!(matches!(
        map_vertex_http_error(401, "unauthorized"),
        CloudError::Auth { .. }
    ));
}

#[test]
fn test_map_vertex_http_error_403_is_auth() {
    assert!(matches!(
        map_vertex_http_error(403, "forbidden"),
        CloudError::Auth { .. }
    ));
}

#[test]
fn test_map_vertex_http_error_429_is_rate_limit() {
    assert!(matches!(
        map_vertex_http_error(429, "quota exceeded"),
        CloudError::RateLimit { .. }
    ));
}

#[test]
fn test_map_vertex_http_error_400_is_not_retryable() {
    assert!(matches!(
        map_vertex_http_error(400, "bad request"),
        CloudError::Provider { retryable: false, .. }
    ));
}

#[test]
fn test_map_vertex_http_error_503_is_retryable() {
    assert!(matches!(
        map_vertex_http_error(503, "unavailable"),
        CloudError::Provider { retryable: true, .. }
    ));
}

#[test]
fn test_map_vertex_http_error_500_is_retryable() {
    assert!(matches!(
        map_vertex_http_error(500, "internal error"),
        CloudError::Provider { retryable: true, .. }
    ));
}

// --- parse_sse_line ---

#[test]
fn test_parse_sse_line_valid_json() {
    let json = parse_sse_line(r#"data: {"candidates": []}"#);
    assert!(json.is_some());
    assert_eq!(json.unwrap()["candidates"], serde_json::json!([]));
}

#[test]
fn test_parse_sse_line_non_data_line_returns_none() {
    assert!(parse_sse_line(": keep-alive").is_none());
}

#[test]
fn test_parse_sse_line_blank_line_returns_none() {
    assert!(parse_sse_line("").is_none());
}

#[test]
fn test_parse_sse_line_invalid_json_returns_none() {
    assert!(parse_sse_line("data: not-json").is_none());
}

// --- sse_chunk_to_events ---

#[test]
fn test_sse_chunk_to_events_text_delta() {
    let json = serde_json::json!({
        "candidates": [{
            "content": { "parts": [{ "text": "Hello" }] },
            "finishReason": "FINISH_REASON_UNSPECIFIED"
        }]
    });
    let events = sse_chunk_to_events(&json);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], LlmStreamEvent::DeltaText(t) if t == "Hello"));
}

#[test]
fn test_sse_chunk_to_events_finish_emits_usage_then_done() {
    let json = serde_json::json!({
        "candidates": [{
            "content": { "parts": [{ "text": "!" }] },
            "finishReason": "STOP"
        }],
        "usageMetadata": { "promptTokenCount": 5, "candidatesTokenCount": 10 }
    });
    let events = sse_chunk_to_events(&json);
    // DeltaText + Usage + Done
    assert_eq!(events.len(), 3);
    assert!(matches!(&events[0], LlmStreamEvent::DeltaText(_)));
    assert!(matches!(&events[1], LlmStreamEvent::Usage(_)));
    assert!(matches!(&events[2], LlmStreamEvent::Done(FinishReason::Stop)));
}

#[test]
fn test_sse_chunk_to_events_finish_without_usage() {
    let json = serde_json::json!({
        "candidates": [{
            "content": { "parts": [{ "text": "" }] },
            "finishReason": "STOP"
        }]
    });
    let events = sse_chunk_to_events(&json);
    // empty text is skipped, no usage, just Done
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], LlmStreamEvent::Done(FinishReason::Stop)));
}

#[test]
fn test_sse_chunk_to_events_unspecified_finish_reason_not_emitted() {
    let json = serde_json::json!({
        "candidates": [{
            "content": { "parts": [{ "text": "hi" }] },
            "finishReason": "FINISH_REASON_UNSPECIFIED"
        }]
    });
    let events = sse_chunk_to_events(&json);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], LlmStreamEvent::DeltaText(_)));
}

#[test]
fn test_sse_chunk_to_events_no_candidates_returns_empty() {
    let json = serde_json::json!({ "candidates": [] });
    assert!(sse_chunk_to_events(&json).is_empty());
}

// --- build_embed_request ---

#[test]
fn test_build_embed_request_single_text() {
    let body = build_embed_request(&["hello".to_string()]);
    assert_eq!(body["instances"][0]["content"], "hello");
}

#[test]
fn test_build_embed_request_multiple_texts() {
    let body = build_embed_request(&["a".to_string(), "b".to_string()]);
    assert_eq!(body["instances"].as_array().unwrap().len(), 2);
    assert_eq!(body["instances"][1]["content"], "b");
}

#[test]
fn test_build_embed_request_empty() {
    let body = build_embed_request(&[]);
    assert_eq!(body["instances"], serde_json::json!([]));
}

// --- parse_embed_response ---

#[test]
fn test_parse_embed_response_single() {
    let json = serde_json::json!({
        "predictions": [{
            "embeddings": { "values": [1.0, 2.0, 3.0] }
        }]
    });
    let resp = parse_embed_response(&json).unwrap();
    assert_eq!(resp.embeddings.len(), 1);
    assert_eq!(resp.embeddings[0], vec![1.0f32, 2.0f32, 3.0f32]);
}

#[test]
fn test_parse_embed_response_multiple() {
    let json = serde_json::json!({
        "predictions": [
            { "embeddings": { "values": [1.0, 2.0] } },
            { "embeddings": { "values": [3.0, 4.0] } }
        ]
    });
    let resp = parse_embed_response(&json).unwrap();
    assert_eq!(resp.embeddings.len(), 2);
    assert_eq!(resp.embeddings[1], vec![3.0f32, 4.0f32]);
}

#[test]
fn test_parse_embed_response_missing_predictions() {
    let json = serde_json::json!({ "error": { "code": 401, "message": "unauthorized" } });
    let err = parse_embed_response(&json).unwrap_err();
    assert!(
        matches!(err, CloudError::Provider { .. }),
        "expected Provider error, got {:?}",
        err
    );
}

#[test]
fn test_parse_embed_response_malformed_values() {
    let json = serde_json::json!({
        "predictions": [{ "embeddings": {} }]
    });
    let err = parse_embed_response(&json).unwrap_err();
    assert!(
        matches!(err, CloudError::Provider { .. }),
        "expected Provider error, got {:?}",
        err
    );
}

// --- async unit tests (no live credentials) ---

#[tokio::test]
async fn test_embed_empty_texts_returns_early() {
    let provider = no_creds_provider();
    let resp = provider.embed(vec![]).await.unwrap();
    assert!(resp.embeddings.is_empty());
}

#[tokio::test]
async fn test_generate_with_tools_returns_unsupported() {
    use crate::types::llm::ToolDefinition;
    let provider = no_creds_provider();
    let req = make_request(
        ModelRef::Provider("gemini-1.5-flash".to_string()),
        vec![Message { role: "user".to_string(), content: "hi".to_string() }],
    );
    let tools = vec![ToolDefinition {
        name: "search".to_string(),
        description: "search the web".to_string(),
        parameters: serde_json::json!({}),
    }];
    let err = provider.generate_with_tools(req, tools).await.unwrap_err();
    assert!(
        matches!(err, CloudError::Unsupported { .. }),
        "expected Unsupported, got {:?}",
        err
    );
}

// --- token caching ---

#[tokio::test]
async fn test_get_token_refreshes_when_expired() {
    use std::sync::atomic::{AtomicU32, Ordering};

    struct CountingProvider {
        count: Arc<AtomicU32>,
        token: String,
    }

    #[async_trait::async_trait]
    impl TokenProvider for CountingProvider {
        async fn get_token(
            &self,
        ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            self.count.fetch_add(1, Ordering::Relaxed);
            Ok(self.token.clone())
        }
    }

    let count = Arc::new(AtomicU32::new(0));
    let auth = Arc::new(CountingProvider {
        count: Arc::clone(&count),
        token: "refreshed".to_string(),
    });

    // with_http_client sets expires_at = Instant::now(), so the first get_token() refreshes
    let provider = VertexAiProvider::with_http_client(
        reqwest::Client::new(),
        "p",
        "us-central1",
        auth,
    );

    let token = provider.get_token().await.unwrap();
    assert_eq!(token, "refreshed");
    assert_eq!(count.load(Ordering::Relaxed), 1);

    // second call is within TTL — no refresh
    let _ = provider.get_token().await.unwrap();
    assert_eq!(count.load(Ordering::Relaxed), 1);
}

// --- integration tests (require live GCP credentials, run with --ignored) ---

#[tokio::test]
#[ignore]
async fn test_embed_live_api() {
    let provider = VertexAiProvider::new("your-project-id", "us-central1")
        .await
        .expect("failed to create provider");
    let resp = provider
        .embed(vec!["Hello, world!".to_string()])
        .await
        .expect("embed failed");
    assert_eq!(resp.embeddings.len(), 1);
    assert!(!resp.embeddings[0].is_empty());
}

#[tokio::test]
#[ignore]
async fn test_generate_live_api() {
    let provider = VertexAiProvider::new("your-project-id", "us-central1")
        .await
        .expect("failed to create provider");
    let req = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash-001".to_string()),
        messages: vec![Message { role: "user".to_string(), content: "What is 2 + 2?".to_string() }],
        max_tokens: Some(64),
        temperature: Some(0.0),
        system_prompt: None,
    };
    let resp = provider.generate(req).await.expect("generate failed");
    assert!(!resp.text.is_empty());
}

#[tokio::test]
#[ignore]
async fn test_stream_live_api() {
    use futures::StreamExt;
    let provider = VertexAiProvider::new("your-project-id", "us-central1")
        .await
        .expect("failed to create provider");
    let req = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash-001".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Count from 1 to 5.".to_string(),
        }],
        max_tokens: Some(64),
        temperature: Some(0.0),
        system_prompt: None,
    };
    let mut stream = provider.stream(req).await.expect("stream failed");
    let mut got_text = false;
    while let Some(event) = stream.next().await {
        match event {
            LlmStreamEvent::DeltaText(_) => got_text = true,
            LlmStreamEvent::Error(e) => panic!("stream error: {:?}", e),
            _ => {}
        }
    }
    assert!(got_text);
}
