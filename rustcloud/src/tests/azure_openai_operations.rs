use futures::StreamExt;

use crate::azure::azure_apis::artificial_intelligence::azure_openai::{
    AzureOpenAiProvider,
    build_chat_request,
    build_embed_request,
    build_tool_request,
    drain_complete_lines,
    emit_chunk_events,
    extract_deployment_name,
    is_reasoning_model,
    map_azure_http_error,
    map_finish_reason,
    parse_azure_error_message,
    parse_chat_response,
    parse_embed_response,
    parse_sse_line,
    parse_tool_response,
    provider_error_from_response,
    pump_stream,
    retry_after_seconds,
    sse_chunk_to_events,
};
use crate::errors::CloudError;
use crate::traits::llm_provider::LlmProvider;
use crate::types::llm::{
    FinishReason, LlmRequest, LlmStreamEvent, Message, ModelRef, ToolCallResponse, ToolDefinition,
};

fn no_creds_provider() -> AzureOpenAiProvider {
    AzureOpenAiProvider::with_http_client(
        reqwest::Client::new(),
        "https://my-resource.openai.azure.com",
        "fake-key",
        "2024-10-21",
        "",
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

// --- azure_endpoint ---

#[test]
fn test_azure_endpoint_format() {
    let provider = no_creds_provider();
    let url = provider.azure_endpoint("gpt-4o", "chat/completions");
    assert_eq!(
        url,
        "https://my-resource.openai.azure.com/openai/deployments/gpt-4o/chat/completions?api-version=2024-10-21"
    );
}

#[test]
fn test_azure_endpoint_trims_trailing_slash() {
    let provider = AzureOpenAiProvider::with_http_client(
        reqwest::Client::new(),
        "https://my-resource.openai.azure.com/",
        "fake-key",
        "2024-10-21",
        "",
    );
    let url = provider.azure_endpoint("gpt-4o", "chat/completions");
    assert_eq!(
        url,
        "https://my-resource.openai.azure.com/openai/deployments/gpt-4o/chat/completions?api-version=2024-10-21"
    );
}

// --- request ---

#[test]
fn test_request_sets_api_key_header() {
    let provider = no_creds_provider();
    let built = provider.request("https://my-resource.openai.azure.com/").build().unwrap();
    assert_eq!(built.headers().get("api-key").unwrap(), "fake-key");
}

// --- extract_deployment_name ---

#[test]
fn test_extract_deployment_name_from_deployment() {
    let result = extract_deployment_name(&ModelRef::Deployment("gpt-4o-mini".to_string()));
    assert_eq!(result.unwrap(), "gpt-4o-mini");
}

#[test]
fn test_extract_deployment_name_provider_alias() {
    let result = extract_deployment_name(&ModelRef::Provider("gpt-4o".to_string()));
    assert_eq!(result.unwrap(), "gpt-4o");
}

#[test]
fn test_extract_deployment_name_rejects_logical() {
    let err = extract_deployment_name(&ModelRef::Logical {
        family: "gpt".to_string(),
        tier: Some("4o".to_string()),
    })
    .unwrap_err();
    assert!(
        matches!(err, CloudError::Unsupported { .. }),
        "expected Unsupported, got {:?}",
        err
    );
}

// --- build_chat_request ---

#[test]
fn test_build_chat_request_includes_system_message() {
    let mut req = make_request(
        ModelRef::Deployment("gpt-4o".to_string()),
        vec![Message { role: "user".to_string(), content: "hi".to_string() }],
    );
    req.system_prompt = Some("You are a helpful assistant.".to_string());
    let body = build_chat_request(&req, "gpt-4o");
    assert_eq!(body["messages"][0]["role"], "system");
    assert_eq!(body["messages"][0]["content"], "You are a helpful assistant.");
    assert_eq!(body["messages"][1]["role"], "user");
    assert_eq!(body["messages"][1]["content"], "hi");
}

#[test]
fn test_build_chat_request_omits_unset_params() {
    let req = make_request(
        ModelRef::Deployment("gpt-4o".to_string()),
        vec![Message { role: "user".to_string(), content: "hi".to_string() }],
    );
    let body = build_chat_request(&req, "gpt-4o");
    assert!(body["max_tokens"].is_null());
    assert!(body["temperature"].is_null());
    assert_eq!(body["messages"][0]["role"], "user");
}

#[test]
fn test_build_chat_request_system_only_no_messages() {
    let mut req = make_request(ModelRef::Deployment("gpt-4o".to_string()), vec![]);
    req.system_prompt = Some("You are a helpful assistant.".to_string());
    let body = build_chat_request(&req, "gpt-4o");
    let messages = body["messages"].as_array().unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0]["role"], "system");
    assert_eq!(messages[0]["content"], "You are a helpful assistant.");
}

#[test]
fn test_build_chat_request_preserves_multi_turn_order() {
    let req = make_request(
        ModelRef::Deployment("gpt-4o".to_string()),
        vec![
            Message { role: "user".to_string(), content: "first".to_string() },
            Message { role: "assistant".to_string(), content: "second".to_string() },
            Message { role: "user".to_string(), content: "third".to_string() },
        ],
    );
    let body = build_chat_request(&req, "gpt-4o");
    let messages = body["messages"].as_array().unwrap();
    assert_eq!(messages.len(), 3);
    assert_eq!(messages[0]["role"], "user");
    assert_eq!(messages[0]["content"], "first");
    assert_eq!(messages[1]["role"], "assistant");
    assert_eq!(messages[1]["content"], "second");
    assert_eq!(messages[2]["role"], "user");
    assert_eq!(messages[2]["content"], "third");
}

#[test]
fn test_is_reasoning_model_o1_o3() {
    assert!(is_reasoning_model("o1-mini"));
    assert!(is_reasoning_model("o3"));
    assert!(is_reasoning_model("O1-PREVIEW"));
    assert!(!is_reasoning_model("gpt-4o"));
    assert!(!is_reasoning_model("gpt-35-turbo"));
}

#[test]
fn test_build_chat_request_reasoning_model_uses_max_completion_tokens() {
    let mut req = make_request(
        ModelRef::Deployment("o1-mini".to_string()),
        vec![Message { role: "user".to_string(), content: "hi".to_string() }],
    );
    req.max_tokens = Some(256);
    let body = build_chat_request(&req, "o1-mini");
    assert!(body["max_tokens"].is_null());
    assert_eq!(body["max_completion_tokens"], 256);
}

#[test]
fn test_build_chat_request_reasoning_model_omits_temperature() {
    let mut req = make_request(
        ModelRef::Deployment("o1-mini".to_string()),
        vec![Message { role: "user".to_string(), content: "hi".to_string() }],
    );
    req.temperature = Some(0.7);
    let body = build_chat_request(&req, "o1-mini");
    assert!(body["temperature"].is_null());
}

// --- map_finish_reason ---

#[test]
fn test_map_finish_reason_stop() {
    assert!(matches!(map_finish_reason("stop"), FinishReason::Stop));
}

#[test]
fn test_map_finish_reason_length() {
    assert!(matches!(map_finish_reason("length"), FinishReason::Length));
}

#[test]
fn test_map_finish_reason_tool_calls() {
    assert!(matches!(map_finish_reason("tool_calls"), FinishReason::ToolCall));
}

#[test]
fn test_map_finish_reason_content_filter() {
    let r = map_finish_reason("content_filter");
    assert!(matches!(r, FinishReason::Other(s) if s == "content_filter"));
}

// --- parse_chat_response ---

#[test]
fn test_parse_chat_response_valid() {
    let json = serde_json::json!({
        "choices": [{
            "message": { "role": "assistant", "content": "Hello!" },
            "finish_reason": "stop"
        }],
        "usage": { "prompt_tokens": 5, "completion_tokens": 2 }
    });
    let resp = parse_chat_response(&json).unwrap();
    assert_eq!(resp.text, "Hello!");
    assert!(matches!(resp.finish_reason, FinishReason::Stop));
    let usage = resp.usage.unwrap();
    assert_eq!(usage.prompt_tokens, 5);
    assert_eq!(usage.completion_tokens, 2);
}

#[test]
fn test_parse_chat_response_missing_choices() {
    let json = serde_json::json!({ "choices": [] });
    let err = parse_chat_response(&json).unwrap_err();
    assert!(
        matches!(err, CloudError::Provider { http_status: 0, .. }),
        "expected parse-error Provider variant, got {:?}",
        err
    );
}

#[test]
fn test_parse_chat_response_no_usage() {
    let json = serde_json::json!({
        "choices": [{
            "message": { "role": "assistant", "content": "ok" },
            "finish_reason": "stop"
        }]
    });
    let resp = parse_chat_response(&json).unwrap();
    assert!(resp.usage.is_none());
}

#[test]
fn test_parse_chat_response_null_content_filtered() {
    let json = serde_json::json!({
        "choices": [{
            "message": { "role": "assistant", "content": null },
            "finish_reason": "content_filter"
        }]
    });
    let resp = parse_chat_response(&json).unwrap();
    assert_eq!(resp.text, "");
    assert!(matches!(resp.finish_reason, FinishReason::Other(s) if s == "content_filter"));
}

// --- retry_after_seconds ---

#[test]
fn test_retry_after_seconds_parses_header() {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::RETRY_AFTER, "30".parse().unwrap());
    assert_eq!(retry_after_seconds(&headers), Some(30));
}

#[test]
fn test_retry_after_seconds_absent() {
    let headers = reqwest::header::HeaderMap::new();
    assert_eq!(retry_after_seconds(&headers), None);
}

#[test]
fn test_retry_after_seconds_non_numeric_returns_none() {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::RETRY_AFTER, "soon".parse().unwrap());
    assert_eq!(retry_after_seconds(&headers), None);
}

// --- parse_azure_error_message ---

#[test]
fn test_parse_azure_error_message_extracts_field() {
    let body = r#"{"error": {"code": "invalid_request", "message": "The api-key you provided is invalid."}}"#;
    assert_eq!(parse_azure_error_message(body), "The api-key you provided is invalid.");
}

#[test]
fn test_parse_azure_error_message_falls_back_on_non_json() {
    assert_eq!(parse_azure_error_message("  not json at all  "), "not json at all");
}

#[test]
fn test_parse_azure_error_message_falls_back_on_missing_field() {
    let body = r#"{"error": {"code": "invalid_request"}}"#;
    assert_eq!(parse_azure_error_message(body), body);
}

// --- map_azure_http_error ---

#[test]
fn test_map_azure_http_error_401_is_auth() {
    assert!(matches!(map_azure_http_error(401, "unauthorized", None), CloudError::Auth { .. }));
}

#[test]
fn test_map_azure_http_error_403_is_auth() {
    assert!(matches!(map_azure_http_error(403, "forbidden", None), CloudError::Auth { .. }));
}

#[test]
fn test_map_azure_http_error_429_reads_retry_after() {
    assert!(matches!(
        map_azure_http_error(429, "quota exceeded", Some(30)),
        CloudError::RateLimit { retry_after: Some(30) }
    ));
}

#[test]
fn test_map_azure_http_error_429_none_when_header_absent() {
    assert!(matches!(
        map_azure_http_error(429, "quota exceeded", None),
        CloudError::RateLimit { retry_after: None }
    ));
}

#[test]
fn test_map_azure_http_error_400_is_not_retryable() {
    assert!(matches!(
        map_azure_http_error(400, "bad request", None),
        CloudError::Provider { retryable: false, .. }
    ));
}

#[test]
fn test_map_azure_http_error_500_is_retryable() {
    assert!(matches!(
        map_azure_http_error(500, "internal error", None),
        CloudError::Provider { retryable: true, .. }
    ));
}

#[test]
fn test_map_azure_http_error_503_is_retryable() {
    assert!(matches!(
        map_azure_http_error(503, "unavailable", None),
        CloudError::Provider { retryable: true, .. }
    ));
}

#[test]
fn test_map_azure_http_error_404_wildcard_not_retryable() {
    assert!(matches!(
        map_azure_http_error(404, "not found", None),
        CloudError::Provider { retryable: false, .. }
    ));
}

// --- parse_sse_line ---

#[test]
fn test_parse_sse_line_data_prefix() {
    let json = parse_sse_line(r#"data: {"choices": []}"#);
    assert!(json.is_some());
    assert_eq!(json.unwrap()["choices"], serde_json::json!([]));
}

#[test]
fn test_parse_sse_line_done_terminator() {
    assert!(parse_sse_line("data: [DONE]").is_none());
}

#[test]
fn test_parse_sse_line_non_data() {
    assert!(parse_sse_line(": keep-alive").is_none());
}

#[test]
fn test_parse_sse_line_invalid_json() {
    assert!(parse_sse_line("data: not-json").is_none());
}

#[test]
fn test_parse_sse_line_blank_line() {
    assert!(parse_sse_line("").is_none());
}

// --- sse_chunk_to_events ---

#[test]
fn test_sse_chunk_delta_text() {
    let json = serde_json::json!({
        "choices": [{ "delta": { "content": "Hello" }, "finish_reason": null }]
    });
    let events = sse_chunk_to_events(&json);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], LlmStreamEvent::DeltaText(t) if t == "Hello"));
}

#[test]
fn test_sse_chunk_finish_reason() {
    let json = serde_json::json!({
        "choices": [{ "delta": {}, "finish_reason": "stop" }]
    });
    let events = sse_chunk_to_events(&json);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], LlmStreamEvent::Done(FinishReason::Stop)));
}

#[test]
fn test_sse_chunk_usage_only_empty_choices() {
    let json = serde_json::json!({
        "choices": [],
        "usage": { "prompt_tokens": 5, "completion_tokens": 10 }
    });
    let events = sse_chunk_to_events(&json);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], LlmStreamEvent::Usage(u) if u.prompt_tokens == 5 && u.completion_tokens == 10));
}

#[test]
fn test_sse_chunk_finish_emits_usage_then_done() {
    let json = serde_json::json!({
        "choices": [{ "delta": {}, "finish_reason": "stop" }],
        "usage": { "prompt_tokens": 5, "completion_tokens": 10 }
    });
    let events = sse_chunk_to_events(&json);
    assert_eq!(events.len(), 2);
    assert!(matches!(&events[0], LlmStreamEvent::Usage(_)));
    assert!(matches!(&events[1], LlmStreamEvent::Done(FinishReason::Stop)));
}

#[test]
fn test_sse_chunk_content_filter_finish() {
    let json = serde_json::json!({
        "choices": [{ "delta": {}, "finish_reason": "content_filter" }]
    });
    let events = sse_chunk_to_events(&json);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], LlmStreamEvent::Done(FinishReason::Other(s)) if s == "content_filter"));
}

#[test]
fn test_sse_chunk_error_object() {
    let json = serde_json::json!({
        "error": { "message": "content filtered", "code": "content_filter" }
    });
    let events = sse_chunk_to_events(&json);
    assert_eq!(events.len(), 1);
    assert!(matches!(
        &events[0],
        LlmStreamEvent::Error(CloudError::Provider { http_status: 200, .. })
    ));
}

#[test]
fn test_sse_chunk_error_object_without_message() {
    let json = serde_json::json!({
        "error": { "code": "content_filter" }
    });
    let events = sse_chunk_to_events(&json);
    assert_eq!(events.len(), 1);
    assert!(matches!(
        &events[0],
        LlmStreamEvent::Error(CloudError::Provider { http_status: 200, .. })
    ));
}

// --- drain_complete_lines ---

#[test]
fn test_drain_complete_lines_handles_multibyte_utf8_split_across_calls() {
    let line = "data: {\"choices\":[{\"delta\":{\"content\":\"caf\u{e9}\"},\"finish_reason\":null}]}\n";
    let bytes = line.as_bytes();
    // split mid two-byte UTF-8 char
    let split_at = bytes.iter().position(|&b| b == 0xC3).unwrap() + 1;

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend_from_slice(&bytes[..split_at]);
    assert!(drain_complete_lines(&mut buffer).is_empty(), "no complete line yet");

    buffer.extend_from_slice(&bytes[split_at..]);
    let parsed = drain_complete_lines(&mut buffer);
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0]["choices"][0]["delta"]["content"], "café");
}

// --- provider_error_from_response (no live credentials) ---

#[tokio::test]
async fn test_provider_error_from_response_maps_status_and_retry_after() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let body = r#"{"error": {"code": "429", "message": "Requests to this deployment exceeded the rate limit."}}"#;

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut discard = [0u8; 1024];
        let _ = socket.read(&mut discard).await;
        let response = format!(
            "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 17\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = socket.write_all(response.as_bytes()).await;
    });

    let response = reqwest::Client::new()
        .get(format!("http://{}/", addr))
        .send()
        .await
        .unwrap();

    let err = provider_error_from_response(response).await;
    assert!(matches!(err, CloudError::RateLimit { retry_after: Some(17) }), "got {:?}", err);
}

// --- emit_chunk_events (direct, no live credentials) ---

#[tokio::test]
async fn test_emit_chunk_events_does_not_reorder_content_after_pending_done() {
    let (mut tx, mut rx) = futures::channel::mpsc::channel::<LlmStreamEvent>(32);
    let mut pending_done: Option<LlmStreamEvent> = None;

    let ok = emit_chunk_events(vec![LlmStreamEvent::Done(FinishReason::Stop)], &mut pending_done, &mut tx).await;
    assert!(ok);
    assert!(pending_done.is_some(), "Done should be buffered, not sent yet");

    let ok = emit_chunk_events(
        vec![LlmStreamEvent::DeltaText("oops".to_string())],
        &mut pending_done,
        &mut tx,
    )
    .await;
    assert!(ok);

    drop(tx);
    let mut received = Vec::new();
    while let Some(event) = rx.next().await {
        received.push(event);
    }

    assert_eq!(received.len(), 1);
    assert!(matches!(&received[0], LlmStreamEvent::DeltaText(t) if t == "oops"));
    assert!(
        pending_done.is_some(),
        "the buffered Done must not jump ahead of unrelated content"
    );
}

#[tokio::test]
async fn test_emit_chunk_events_flushes_prior_done_before_storing_new_one() {
    let (mut tx, mut rx) = futures::channel::mpsc::channel::<LlmStreamEvent>(32);
    let mut pending_done: Option<LlmStreamEvent> = None;

    let ok = emit_chunk_events(vec![LlmStreamEvent::Done(FinishReason::Stop)], &mut pending_done, &mut tx).await;
    assert!(ok);
    let ok = emit_chunk_events(vec![LlmStreamEvent::Done(FinishReason::Length)], &mut pending_done, &mut tx).await;
    assert!(ok);

    drop(tx);
    let mut received = Vec::new();
    while let Some(event) = rx.next().await {
        received.push(event);
    }

    assert_eq!(received.len(), 1, "the first Done must be flushed, not silently dropped");
    assert!(matches!(&received[0], LlmStreamEvent::Done(FinishReason::Stop)));
    assert!(matches!(pending_done, Some(LlmStreamEvent::Done(FinishReason::Length))));
}

// --- pump_stream edge cases (no live credentials) ---

#[tokio::test]
async fn test_stream_connection_drop() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        // unread request bytes on close cause an RST, not a truncated body
        let mut discard = [0u8; 1024];
        let _ = socket.read(&mut discard).await;
        let _ = socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\n\r\nshort")
            .await;
    });

    let response = reqwest::Client::new()
        .get(format!("http://{}/", addr))
        .send()
        .await
        .unwrap();

    let (tx, mut rx) = futures::channel::mpsc::channel::<LlmStreamEvent>(32);
    pump_stream(response, tx).await;

    match rx.next().await {
        Some(LlmStreamEvent::Error(CloudError::Network { .. })) => {}
        other => panic!("expected a Network error event, got {:?}", other),
    }
    assert!(rx.next().await.is_none());
}

#[tokio::test]
async fn test_stream_flushes_pending_done_on_connection_drop() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // finish_reason arrives (buffered as pending_done awaiting a usage chunk),
    // then the connection is cut before the usage chunk or a clean close arrives
    let body = "data: {\"choices\":[{\"delta\":{},\"finish_reason\":\"stop\"}]}\n";

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut discard = [0u8; 1024];
        let _ = socket.read(&mut discard).await;
        // Content-Length overstates the body so the connection closing after it
        // is read as a truncated body (Err), not a clean end-of-stream (Ok(None)).
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: 1000\r\n\r\n{}", body);
        let _ = socket.write_all(response.as_bytes()).await;
    });

    let response = reqwest::Client::new()
        .get(format!("http://{}/", addr))
        .send()
        .await
        .unwrap();

    let (tx, mut rx) = futures::channel::mpsc::channel::<LlmStreamEvent>(32);
    pump_stream(response, tx).await;

    match rx.next().await {
        Some(LlmStreamEvent::Done(FinishReason::Stop)) => {}
        other => panic!("expected the buffered Done to survive the connection drop, got {:?}", other),
    }
    assert!(
        rx.next().await.is_none(),
        "a completed generation should not also surface a stream error"
    );
}

#[tokio::test]
async fn test_stream_flushes_trailing_line_without_terminator() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // no trailing newline before the clean close
    let body = r#"data: {"choices":[{"delta":{"content":"hi"},"finish_reason":null}]}"#;
    let body_len = body.len();

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut discard = [0u8; 1024];
        let _ = socket.read(&mut discard).await;
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body_len, body);
        let _ = socket.write_all(response.as_bytes()).await;
    });

    let response = reqwest::Client::new()
        .get(format!("http://{}/", addr))
        .send()
        .await
        .unwrap();

    let (tx, mut rx) = futures::channel::mpsc::channel::<LlmStreamEvent>(32);
    pump_stream(response, tx).await;

    match rx.next().await {
        Some(LlmStreamEvent::DeltaText(t)) => assert_eq!(t, "hi"),
        other => panic!("expected DeltaText(\"hi\"), got {:?}", other),
    }
}

#[tokio::test]
async fn test_stream_orders_usage_before_done_across_separate_chunks() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // real Azure/OpenAI wire behavior: finish_reason and usage arrive as two
    // separate chunks, not combined in one JSON payload
    let finish_line = r#"data: {"choices":[{"delta":{},"finish_reason":"stop"}]}"#;
    let usage_line = r#"data: {"choices":[],"usage":{"prompt_tokens":5,"completion_tokens":10}}"#;
    let body = format!("{}\n{}\n", finish_line, usage_line);
    let body_len = body.len();

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut discard = [0u8; 1024];
        let _ = socket.read(&mut discard).await;
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body_len, body);
        let _ = socket.write_all(response.as_bytes()).await;
    });

    let response = reqwest::Client::new()
        .get(format!("http://{}/", addr))
        .send()
        .await
        .unwrap();

    let (tx, mut rx) = futures::channel::mpsc::channel::<LlmStreamEvent>(32);
    pump_stream(response, tx).await;

    let mut events = Vec::new();
    while let Some(event) = rx.next().await {
        events.push(event);
    }

    assert_eq!(events.len(), 2);
    assert!(matches!(&events[0], LlmStreamEvent::Usage(u) if u.prompt_tokens == 5 && u.completion_tokens == 10));
    assert!(matches!(&events[1], LlmStreamEvent::Done(FinishReason::Stop)));
}

#[tokio::test]
async fn test_stream_flushes_pending_done_on_clean_close_without_usage() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // server closes right after finish_reason; no stream_options usage chunk ever arrives
    let body = "data: {\"choices\":[{\"delta\":{},\"finish_reason\":\"stop\"}]}\n";
    let body_len = body.len();

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut discard = [0u8; 1024];
        let _ = socket.read(&mut discard).await;
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body_len, body);
        let _ = socket.write_all(response.as_bytes()).await;
    });

    let response = reqwest::Client::new()
        .get(format!("http://{}/", addr))
        .send()
        .await
        .unwrap();

    let (tx, mut rx) = futures::channel::mpsc::channel::<LlmStreamEvent>(32);
    pump_stream(response, tx).await;

    let mut events = Vec::new();
    while let Some(event) = rx.next().await {
        events.push(event);
    }

    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], LlmStreamEvent::Done(FinishReason::Stop)));
}

// --- build_embed_request ---

#[test]
fn test_build_embed_request() {
    let texts = vec!["hello".to_string(), "world".to_string()];
    let body = build_embed_request(&texts);
    assert_eq!(body["input"], serde_json::json!(["hello", "world"]));
}

// --- parse_embed_response ---

#[test]
fn test_parse_embed_response_valid() {
    let json = serde_json::json!({
        "data": [
            { "index": 0, "embedding": [0.1, 0.2] },
            { "index": 1, "embedding": [0.3, 0.4] }
        ]
    });
    let resp = parse_embed_response(&json).unwrap();
    assert_eq!(resp.embeddings, vec![vec![0.1, 0.2], vec![0.3, 0.4]]);
}

#[test]
fn test_parse_embed_response_reorders_by_index() {
    let json = serde_json::json!({
        "data": [
            { "index": 1, "embedding": [0.3, 0.4] },
            { "index": 0, "embedding": [0.1, 0.2] }
        ]
    });
    let resp = parse_embed_response(&json).unwrap();
    assert_eq!(resp.embeddings, vec![vec![0.1, 0.2], vec![0.3, 0.4]]);
}

#[test]
fn test_parse_embed_response_missing_data_key() {
    let json = serde_json::json!({});
    let err = parse_embed_response(&json).unwrap_err();
    assert!(matches!(err, CloudError::Provider { http_status: 0, .. }));
}

#[test]
fn test_parse_embed_response_missing_index_is_error() {
    let json = serde_json::json!({
        "data": [
            { "embedding": [0.1, 0.2] },
            { "index": 1, "embedding": [0.3, 0.4] }
        ]
    });
    let err = parse_embed_response(&json).unwrap_err();
    assert!(matches!(err, CloudError::Provider { http_status: 0, .. }));
}

#[test]
fn test_parse_embed_response_duplicate_index_is_error() {
    let json = serde_json::json!({
        "data": [
            { "index": 0, "embedding": [0.1, 0.2] },
            { "index": 0, "embedding": [0.3, 0.4] }
        ]
    });
    let err = parse_embed_response(&json).unwrap_err();
    assert!(matches!(err, CloudError::Provider { http_status: 0, .. }));
}

#[test]
fn test_parse_embed_response_out_of_range_index_is_error() {
    let json = serde_json::json!({
        "data": [
            { "index": 5, "embedding": [0.1, 0.2] }
        ]
    });
    let err = parse_embed_response(&json).unwrap_err();
    assert!(matches!(err, CloudError::Provider { http_status: 0, .. }));
}

// --- embed (no live credentials) ---

#[tokio::test]
async fn test_embed_empty_input_no_http_call() {
    let provider = no_creds_provider();
    let resp = provider.embed(vec![]).await.unwrap();
    assert!(resp.embeddings.is_empty());
}

// --- build_tool_request ---

fn make_tool() -> ToolDefinition {
    ToolDefinition {
        name: "get_weather".to_string(),
        description: "Gets the weather for a city".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": { "city": { "type": "string" } },
            "required": ["city"]
        }),
    }
}

#[test]
fn test_build_tool_request_shape() {
    let req = make_request(
        ModelRef::Deployment("gpt-4o".to_string()),
        vec![Message { role: "user".to_string(), content: "weather in Pune?".to_string() }],
    );
    let body = build_tool_request(&req, "gpt-4o", &[make_tool()]);

    assert_eq!(body["tool_choice"], "auto");
    assert_eq!(body["tools"][0]["type"], "function");
    assert_eq!(body["tools"][0]["function"]["name"], "get_weather");
    assert_eq!(
        body["tools"][0]["function"]["description"],
        "Gets the weather for a city"
    );
    assert_eq!(body["tools"][0]["function"]["parameters"]["type"], "object");
}

#[test]
fn test_build_tool_request_disables_parallel_tool_calls() {
    let req = make_request(
        ModelRef::Deployment("gpt-4o".to_string()),
        vec![Message { role: "user".to_string(), content: "weather in Pune?".to_string() }],
    );
    let body = build_tool_request(&req, "gpt-4o", &[make_tool()]);
    assert_eq!(body["parallel_tool_calls"], false);
}

#[test]
fn test_build_tool_request_empty_tools_omits_key() {
    let req = make_request(
        ModelRef::Deployment("gpt-4o".to_string()),
        vec![Message { role: "user".to_string(), content: "hi".to_string() }],
    );
    let body = build_tool_request(&req, "gpt-4o", &[]);
    assert!(body["tools"].is_null());
    assert!(body["tool_choice"].is_null());
}

#[test]
fn test_build_tool_request_preserves_chat_fields() {
    let mut req = make_request(
        ModelRef::Deployment("gpt-4o".to_string()),
        vec![Message { role: "user".to_string(), content: "weather in Pune?".to_string() }],
    );
    req.max_tokens = Some(128);
    req.system_prompt = Some("You are terse.".to_string());
    let body = build_tool_request(&req, "gpt-4o", &[make_tool()]);

    assert_eq!(body["max_tokens"], 128);
    assert_eq!(body["messages"][0]["role"], "system");
    assert_eq!(body["messages"][1]["content"], "weather in Pune?");
}

// --- parse_tool_response ---

#[test]
fn test_parse_tool_response_parses_stringified_arguments() {
    let json = serde_json::json!({
        "choices": [{
            "message": {
                "role": "assistant",
                "tool_calls": [{
                    "function": {
                        "name": "get_weather",
                        "arguments": "{\"city\":\"Pune\"}"
                    }
                }]
            },
            "finish_reason": "tool_calls"
        }]
    });
    let resp = parse_tool_response(&json).unwrap();
    match resp {
        ToolCallResponse::ToolCall { name, arguments } => {
            assert_eq!(name, "get_weather");
            assert_eq!(arguments["city"], "Pune");
        }
        other => panic!("expected ToolCall, got {:?}", other),
    }
}

#[test]
fn test_parse_tool_response_malformed_arguments_string() {
    let json = serde_json::json!({
        "choices": [{
            "message": {
                "role": "assistant",
                "tool_calls": [{
                    "function": {
                        "name": "get_weather",
                        "arguments": "not-json"
                    }
                }]
            },
            "finish_reason": "tool_calls"
        }]
    });
    let err = parse_tool_response(&json).unwrap_err();
    assert!(matches!(err, CloudError::Provider { http_status: 0, .. }));
}

#[test]
fn test_parse_tool_response_text_fallback() {
    let json = serde_json::json!({
        "choices": [{
            "message": { "role": "assistant", "content": "It's sunny." },
            "finish_reason": "stop"
        }]
    });
    let resp = parse_tool_response(&json).unwrap();
    match resp {
        ToolCallResponse::Text(inner) => assert_eq!(inner.text, "It's sunny."),
        other => panic!("expected Text, got {:?}", other),
    }
}

#[test]
fn test_tool_call_undefined_tool() {
    let json = serde_json::json!({
        "choices": [{
            "message": {
                "role": "assistant",
                "tool_calls": [{
                    "function": {
                        "name": "some_tool_not_in_request",
                        "arguments": "{}"
                    }
                }]
            },
            "finish_reason": "tool_calls"
        }]
    });
    let resp = parse_tool_response(&json).unwrap();
    assert!(matches!(resp, ToolCallResponse::ToolCall { name, .. } if name == "some_tool_not_in_request"));
}

// --- generate_with_tools (no live credentials) ---

#[tokio::test]
async fn test_generate_with_tools_rejects_empty_tools() {
    let provider = no_creds_provider();
    let req = make_request(
        ModelRef::Deployment("gpt-4o".to_string()),
        vec![Message { role: "user".to_string(), content: "hi".to_string() }],
    );
    let err = provider.generate_with_tools(req, vec![]).await.unwrap_err();
    assert!(matches!(err, CloudError::Unsupported { .. }));
}

// --- integration tests (require live Azure OpenAI credentials, run with --ignored) ---

#[tokio::test]
#[ignore]
async fn test_generate_live() {
    let provider = AzureOpenAiProvider::new().expect("failed to create provider");
    let req = LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
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
async fn test_generate_live_invalid_key() {
    let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT").expect("AZURE_OPENAI_ENDPOINT not set");
    let provider = AzureOpenAiProvider::with_http_client(
        reqwest::Client::new(),
        endpoint,
        "invalid-key",
        "2024-10-21",
        "",
    );
    let req = make_request(
        ModelRef::Deployment("gpt-4o".to_string()),
        vec![Message { role: "user".to_string(), content: "hi".to_string() }],
    );
    let err = provider.generate(req).await.expect_err("expected an auth error with an invalid key");
    match err {
        CloudError::Auth { message } => assert!(!message.is_empty()),
        other => panic!("expected CloudError::Auth, got {:?}", other),
    }
}

#[tokio::test]
#[ignore]
async fn test_stream_live() {
    let provider = AzureOpenAiProvider::new().expect("failed to create provider");
    let req = LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
        messages: vec![Message { role: "user".to_string(), content: "Count from 1 to 5.".to_string() }],
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

#[tokio::test]
#[ignore]
async fn test_embed_live() {
    let provider = AzureOpenAiProvider::new().expect("failed to create provider");
    let resp = provider
        .embed(vec!["hello world".to_string()])
        .await
        .expect("embed failed");
    assert_eq!(resp.embeddings.len(), 1);
    assert!(!resp.embeddings[0].is_empty());
}

#[tokio::test]
#[ignore]
async fn test_generate_with_tools_live() {
    let provider = AzureOpenAiProvider::new().expect("failed to create provider");
    let req = LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What's the weather in Pune?".to_string(),
        }],
        max_tokens: Some(64),
        temperature: Some(0.0),
        system_prompt: None,
    };
    let tools = vec![ToolDefinition {
        name: "get_weather".to_string(),
        description: "Gets the weather for a city".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": { "city": { "type": "string" } },
            "required": ["city"]
        }),
    }];
    let resp = provider
        .generate_with_tools(req, tools)
        .await
        .expect("generate_with_tools failed");
    assert!(matches!(resp, ToolCallResponse::ToolCall { .. }));
}
