# Unified GenAI Client

RustCloud provides a single `UnifiedLlmClient` that routes generative AI
requests across AWS Bedrock, GCP Vertex AI, and Azure OpenAI through a shared
`LlmProvider` interface — with no provider-specific code in your application.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    UnifiedLlmClient                          │
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────┐   │
│  │  BedrockProvider│  │  VertexAI    │  │AzureOpenAI     │   │
│  │  (aws)       │  │  (gcp)       │  │Provider (azure)│   │
│  └──────────────┘  └──────────────┘  └────────────────┘   │
│                                                             │
│  RoutingStrategy:  Explicit | ModelBased | Fallback         │
└─────────────────────────────────────────────────────────────┘
```

All three backends implement the same `LlmProvider` trait:

| Method                  | What it does                              |
|-------------------------|-------------------------------------------|
| `generate`              | Single-turn text completion               |
| `stream`                | Server-sent-event token streaming         |
| `embed`                 | Batch text embeddings                     |
| `generate_with_tools`   | Function / tool calling                   |

---

## Credentials

Each backend reads its credentials from environment variables:

**AWS Bedrock**
```bash
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...
export AWS_DEFAULT_REGION=us-east-1
```

**GCP Vertex AI**
```bash
export GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account.json
# or use: gcloud auth application-default login
export GCP_PROJECT_ID=my-project
export GCP_LOCATION=us-central1
```

**Azure OpenAI**
```bash
export AZURE_OPENAI_ENDPOINT=https://my-resource.openai.azure.com
export AZURE_OPENAI_API_KEY=...
export AZURE_OPENAI_API_VERSION=2024-10-21       # optional, has default
export AZURE_OPENAI_EMBED_DEPLOYMENT=text-embedding-ada-002  # optional
```

---

## Strategy 1 — Explicit (default)

Always routes to the registered default provider. Good for single-provider
setups or when you want deterministic routing with a manual override path.

```rust
use rustcloud::genai::client::UnifiedLlmClient;
use rustcloud::genai::routing::RoutingStrategy;
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
use rustcloud::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UnifiedLlmClient::builder()
        .register("aws",   Box::new(BedrockProvider::new().await))
        .register("azure", Box::new(AzureOpenAIProvider::new()))
        .default_provider("azure")          // always use Azure
        .routing(RoutingStrategy::Explicit) // explicit is also the default
        .build()?;

    let response = client.generate(LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Summarise the Rust ownership model in one sentence.".to_string(),
        }],
        max_tokens: Some(100),
        temperature: Some(0.3),
        system_prompt: Some("You are a concise technical writer.".to_string()),
    }).await?;

    println!("{}", response.text);
    Ok(())
}
```

---

## Strategy 2 — ModelBased (recommended for multi-cloud)

Inspects the `ModelRef` and routes automatically:

| Model prefix / pattern                   | Routed to  |
|------------------------------------------|------------|
| `anthropic.*`, `amazon.*`, `meta.*`      | `"aws"`    |
| `gemini*`, `text-embedding-*`            | `"gcp"`    |
| `gpt-*`, `o1*`, `o3*`, `Deployment(_)`  | `"azure"`  |
| No match                                 | default    |

```rust
use rustcloud::genai::client::UnifiedLlmClient;
use rustcloud::genai::routing::RoutingStrategy;
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
use rustcloud::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;
use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAI;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UnifiedLlmClient::builder()
        .register("aws",   Box::new(BedrockProvider::new().await))
        .register("gcp",   Box::new(VertexAI::new("my-project", "us-central1")))
        .register("azure", Box::new(AzureOpenAIProvider::new()))
        .routing(RoutingStrategy::ModelBased)
        .build()?;

    // Automatically routed to AWS Bedrock (anthropic.* prefix)
    let bedrock_resp = client.generate(LlmRequest {
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
        messages: vec![Message { role: "user".to_string(), content: "Hello!".to_string() }],
        max_tokens: Some(50),
        temperature: None,
        system_prompt: None,
    }).await?;
    println!("Bedrock: {}", bedrock_resp.text);

    // Automatically routed to GCP Vertex AI (gemini* prefix)
    let vertex_resp = client.generate(LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash".to_string()),
        messages: vec![Message { role: "user".to_string(), content: "Hello!".to_string() }],
        max_tokens: Some(50),
        temperature: None,
        system_prompt: None,
    }).await?;
    println!("Vertex AI: {}", vertex_resp.text);

    // Automatically routed to Azure (Deployment variant always → azure)
    let azure_resp = client.generate(LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
        messages: vec![Message { role: "user".to_string(), content: "Hello!".to_string() }],
        max_tokens: Some(50),
        temperature: None,
        system_prompt: None,
    }).await?;
    println!("Azure: {}", azure_resp.text);

    Ok(())
}
```

---

## Strategy 3 — Fallback (resilience / high availability)

Tries providers in registration order. Moves to the next on transient errors
(`RateLimit`, `Network`, `Provider { retryable: true }`). Hard errors (`Auth`,
`Unsupported`, non-retryable `Provider`) are also skipped and the next
provider is tried. If all providers fail the last error is returned.

```rust
use rustcloud::genai::client::UnifiedLlmClient;
use rustcloud::genai::routing::RoutingStrategy;
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
use rustcloud::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;
use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAI;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // AWS is tried first; if rate-limited → GCP; if rate-limited → Azure
    let client = UnifiedLlmClient::builder()
        .register("aws",   Box::new(BedrockProvider::new().await))
        .register("gcp",   Box::new(VertexAI::new("my-project", "us-central1")))
        .register("azure", Box::new(AzureOpenAIProvider::new()))
        .routing(RoutingStrategy::Fallback)
        .build()?;

    let response = client.generate(LlmRequest {
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What is 1 + 1?".to_string(),
        }],
        max_tokens: Some(20),
        temperature: Some(0.0),
        system_prompt: None,
    }).await?;

    println!("Answer: {}", response.text);
    Ok(())
}
```

---

## Streaming

```rust
use futures::StreamExt;
use rustcloud::types::llm::LlmStreamEvent;

// ... build client as above ...

let mut stream = client.stream(LlmRequest {
    model: ModelRef::Provider("gemini-1.5-flash".to_string()),
    messages: vec![Message {
        role: "user".to_string(),
        content: "Write a haiku about Rust.".to_string(),
    }],
    max_tokens: Some(60),
    temperature: Some(0.7),
    system_prompt: None,
}).await?;

while let Some(event) = stream.next().await {
    match event {
        LlmStreamEvent::DeltaText(chunk) => print!("{}", chunk),
        LlmStreamEvent::Done(reason)     => println!("\n[done: {:?}]", reason),
        LlmStreamEvent::Usage(u)         => println!("[tokens: {}p {}c]",
                                                u.prompt_tokens, u.completion_tokens),
        LlmStreamEvent::Error(e)         => eprintln!("[stream error: {}]", e),
    }
}
```

---

## Embeddings

```rust
let embed_resp = client.embed(vec![
    "Rust is a systems programming language.".to_string(),
    "Python is great for data science.".to_string(),
    "Go has fast compilation.".to_string(),
]).await?;

for (i, vec) in embed_resp.embeddings.iter().enumerate() {
    println!("Text {}: {} dimensions", i, vec.len());
}
```

> **Note:** `embed()` always uses the `default_provider` for `Explicit` and
> `ModelBased` strategies (there is no `ModelRef` to infer from). With
> `Fallback`, it tries providers in order.

---

## Function Calling / Tool Use

```rust
use rustcloud::types::llm::{ToolCallResponse, ToolDefinition};
use serde_json::json;

let tools = vec![ToolDefinition {
    name: "get_weather".to_string(),
    description: "Returns current weather for a city.".to_string(),
    parameters: json!({
        "type": "object",
        "properties": {
            "city": { "type": "string", "description": "City name" }
        },
        "required": ["city"]
    }),
}];

let result = client.generate_with_tools(LlmRequest {
    model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
    messages: vec![Message {
        role: "user".to_string(),
        content: "What is the weather in Tokyo?".to_string(),
    }],
    max_tokens: Some(200),
    temperature: Some(0.0),
    system_prompt: None,
}, tools).await?;

match result {
    ToolCallResponse::ToolCall { name, arguments } => {
        println!("Tool: {}", name);
        println!("Args: {}", arguments);
    }
    ToolCallResponse::Text(resp) => {
        println!("Text: {}", resp.text);
    }
}
```

---

## Routing decision table

| Strategy     | `generate` / `stream` / `generate_with_tools` | `embed`               |
|--------------|------------------------------------------------|-----------------------|
| `Explicit`   | default provider                               | default provider      |
| `ModelBased` | inferred from `ModelRef`, fallback to default  | default provider      |
| `Fallback`   | providers in registration order                | providers in order    |

---

## Error handling

```rust
use rustcloud::errors::CloudError;

match client.generate(req).await {
    Ok(resp) => println!("{}", resp.text),
    Err(CloudError::Auth { message })          => eprintln!("Auth: {}", message),
    Err(CloudError::RateLimit { retry_after }) => {
        eprintln!("Rate limited; retry after {:?}s", retry_after);
    }
    Err(CloudError::Provider { http_status, message, retryable }) => {
        eprintln!("Provider {} (retryable={}): {}", http_status, retryable, message);
    }
    Err(CloudError::Network { source })        => eprintln!("Network: {}", source),
    Err(e)                                     => eprintln!("Error: {}", e),
}
```

---

## References

- [LlmProvider trait](../../rustcloud/src/traits/llm_provider.rs)
- [UnifiedLlmClient](../../rustcloud/src/genai/client.rs)
- [RoutingStrategy](../../rustcloud/src/genai/routing.rs)
- [AWS Bedrock example](../aws/artificial_intelligence/bedrock.md)
- [GCP Vertex AI example](../gcp/artificial_intelligence/vertex_ai.md)
- [Azure OpenAI example](../azure/artificial_intelligence/azure_openai.md)
