use crate::gcp::gcp_apis::artificial_intelligence::vertex::GoogleVertexAI;
use crate::types::llm::{LlmRequest, Message, ModelRef};

#[tokio::test]
async fn test_vertex_compilation() {
    let _client = GoogleVertexAI::new("test-project".to_string(), None);
    let req = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello from Vertex AI".to_string(),
        }],
        max_tokens: Some(50),
        temperature: Some(0.7),
        system_prompt: None,
    };
    
    assert_eq!(req.model, ModelRef::Provider("gemini-1.5-flash".to_string()));
}
