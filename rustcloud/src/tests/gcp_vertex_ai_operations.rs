use crate::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAI;
use crate::traits::llm_provider::LlmProvider;
use crate::types::llm::{LlmRequest, Message, ModelRef, ToolDefinition};

fn create_client() -> VertexAI {
    VertexAI::new("your_project_id", "us-central1")
}

// ─── Dataset Management Tests ───

#[tokio::test]
async fn test_create_dataset() {
    let client = create_client();

    let display_name = "test-dataset";
    let schema_uri = "gs://google-cloud-aiplatform/schema/dataset/metadata/tabular_1.0.0.yaml";

    let result = client.create_dataset(display_name, schema_uri).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_dataset() {
    let client = create_client();

    let dataset_id = "your_dataset_id";

    let result = client.get_dataset(dataset_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_import_data() {
    let client = create_client();

    let dataset_id = "your_dataset_id";
    let gcs_uris = vec!["gs://your_bucket/your_file.csv".to_string()];
    let schema_uri = "gs://google-cloud-aiplatform/schema/dataset/ioformat/tabular_1.0.0.yaml";

    let result = client.import_data(dataset_id, gcs_uris, schema_uri).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_export_dataset() {
    let client = create_client();

    let dataset_id = "your_dataset_id";
    let gcs_uri = "gs://your_bucket/your_export_path/";

    let result = client.export_dataset(dataset_id, gcs_uri).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_dataset() {
    let client = create_client();

    let dataset_id = "your_dataset_id";

    let result = client.delete_dataset(dataset_id).await;
    assert!(result.is_ok());
}

// ─── Model Management Tests ───

#[tokio::test]
async fn test_list_models() {
    let client = create_client();

    let result = client.list_models().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_model() {
    let client = create_client();

    let model_id = "your_model_id";

    let result = client.get_model(model_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_model() {
    let client = create_client();

    let model_id = "your_model_id";

    let result = client.delete_model(model_id).await;
    assert!(result.is_ok());
}

// ─── Endpoint Management Tests ───

#[tokio::test]
async fn test_create_endpoint() {
    let client = create_client();

    let display_name = "test-endpoint";

    let result = client.create_endpoint(display_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deploy_model() {
    let client = create_client();

    let endpoint_id = "your_endpoint_id";
    let model_resource_name = "projects/your_project/locations/us-central1/models/your_model";
    let display_name = "deployed-model";

    let result = client
        .deploy_model(endpoint_id, model_resource_name, display_name)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_undeploy_model() {
    let client = create_client();

    let endpoint_id = "your_endpoint_id";
    let deployed_model_id = "your_deployed_model_id";

    let result = client
        .undeploy_model(endpoint_id, deployed_model_id)
        .await;
    assert!(result.is_ok());
}

// ─── LlmProvider Trait Tests ───

#[tokio::test]
async fn test_generate() {
    let client = create_client();

    let req = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello, Gemini! Say hello back in one sentence.".to_string(),
        }],
        max_tokens: Some(100),
        temperature: Some(0.7),
        system_prompt: None,
    };

    let result = client.generate(req).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_generate_with_system_prompt() {
    let client = create_client();

    let req = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What is 2+2?".to_string(),
        }],
        max_tokens: Some(50),
        temperature: Some(0.0),
        system_prompt: Some("You are a math tutor. Answer concisely.".to_string()),
    };

    let result = client.generate(req).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_embed() {
    let client = create_client();

    let texts = vec![
        "Hello world".to_string(),
        "Rust is great".to_string(),
    ];

    let result = client.embed(texts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_generate_with_tools() {
    let client = create_client();

    let req = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What is the weather in London?".to_string(),
        }],
        max_tokens: Some(100),
        temperature: None,
        system_prompt: None,
    };

    let tools = vec![ToolDefinition {
        name: "get_weather".to_string(),
        description: "Get current weather for a location".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city name"
                }
            },
            "required": ["location"]
        }),
    }];

    let result = client.generate_with_tools(req, tools).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_logical_model_ref() {
    let client = create_client();

    let req = LlmRequest {
        model: ModelRef::Logical {
            family: "gemini".to_string(),
            tier: Some("flash".to_string()),
        },
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello!".to_string(),
        }],
        max_tokens: Some(50),
        temperature: None,
        system_prompt: None,
    };

    let result = client.generate(req).await;
    assert!(result.is_ok());
}
