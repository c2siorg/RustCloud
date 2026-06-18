
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

## Embed text and tool calling

`embed()` and `generate_with_tools()` are not yet available for `VertexAiProvider`. They will be added in a future release.
