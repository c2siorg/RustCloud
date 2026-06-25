
# rustcloud - GCP Vertex AI

## Configure GCP credentials

Place your service account key file at `service-account.json` in the root of your project:

```sh
gcloud iam service-accounts keys create service-account.json \
  --iam-account=my-sa@my-project.iam.gserviceaccount.com
```

The service account must have the `roles/aiplatform.user` IAM role on the project.

## Initialize the provider

```rust
use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAiProvider;

#[tokio::main]
async fn main() {
    let provider = VertexAiProvider::new("my-gcp-project", "us-central1")
        .await
        .expect("failed to authenticate with Vertex AI");
}
```

## Generate text

```rust
use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAiProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef};

#[tokio::main]
async fn main() {
    let provider = VertexAiProvider::new("my-gcp-project", "us-central1")
        .await
        .expect("failed to authenticate with Vertex AI");

    let req = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash-001".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Explain what a Rust lifetime is.".to_string(),
        }],
        max_tokens: Some(256),
        temperature: Some(0.5),
        system_prompt: Some("You are a concise technical writer.".to_string()),
    };

    let response = provider.generate(req).await.unwrap();
    println!("{}", response.text);
}
```

Use `ModelRef::Logical` to let the provider resolve a model family and tier:

```rust
ModelRef::Logical {
    family: "gemini".to_string(),
    tier: Some("1.5-pro".to_string()),
}
```

`ModelRef::Deployment` is not supported by Vertex AI and returns an error.

## Stream a response

```rust
use futures::StreamExt;
use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAiProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, LlmStreamEvent, Message, ModelRef};

#[tokio::main]
async fn main() {
    let provider = VertexAiProvider::new("my-gcp-project", "us-central1")
        .await
        .expect("failed to authenticate with Vertex AI");

    let req = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash-001".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "List five uses of Rust in production systems.".to_string(),
        }],
        max_tokens: Some(512),
        temperature: None,
        system_prompt: None,
    };

    let mut stream = provider.stream(req).await.unwrap();

    while let Some(event) = stream.next().await {
        match event {
            LlmStreamEvent::DeltaText(chunk) => print!("{}", chunk),
            LlmStreamEvent::Done(_) => println!(),
            LlmStreamEvent::Error(e) => eprintln!("stream error: {:?}", e),
            _ => {}
        }
    }
}
```

The OAuth2 bearer token is refreshed automatically before any request if it expires within five minutes. Token refresh never interrupts an in-flight stream.

## Embed text

```rust
use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAiProvider;
use rustcloud::traits::llm_provider::LlmProvider;

#[tokio::main]
async fn main() {
    let provider = VertexAiProvider::new("my-gcp-project", "us-central1")
        .await
        .expect("failed to authenticate with Vertex AI");

    let response = provider
        .embed(vec!["The quick brown fox".to_string()])
        .await
        .unwrap();

    println!("dimensions: {}", response.embeddings[0].len());
}
```

Pass multiple strings to embed them in a single request:

```rust
let texts = vec![
    "first document".to_string(),
    "second document".to_string(),
    "third document".to_string(),
];

let response = provider.embed(texts).await.unwrap();

for (i, emb) in response.embeddings.iter().enumerate() {
    println!("text {}: {} dimensions", i, emb.len());
}
```

The model is fixed to `text-embedding-004`. Passing an empty slice returns immediately without a network request.

## Call tools

```rust
use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAiProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef, ToolCallResponse, ToolDefinition};

#[tokio::main]
async fn main() {
    let provider = VertexAiProvider::new("my-gcp-project", "us-central1")
        .await
        .expect("failed to authenticate with Vertex AI");

    let tools = vec![ToolDefinition {
        name: "get_weather".to_string(),
        description: "Returns the current weather for a given city.".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "city": { "type": "string", "description": "City name" }
            },
            "required": ["city"]
        }),
    }];

    let req = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash-001".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What is the weather in London?".to_string(),
        }],
        max_tokens: Some(256),
        temperature: Some(0.0),
        system_prompt: None,
    };

    match provider.generate_with_tools(req, tools).await.unwrap() {
        ToolCallResponse::ToolCall { name, arguments } => {
            println!("tool: {}, args: {}", name, arguments);
        }
        ToolCallResponse::Text(resp) => {
            println!("{}", resp.text);
        }
    }
}
```

When the model calls a tool, the response is `ToolCallResponse::ToolCall` with the tool name and a JSON value of its arguments. If the model replies with text instead, it falls back to `ToolCallResponse::Text`.
