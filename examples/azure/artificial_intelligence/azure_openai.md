# Azure OpenAI Service Integration

This guide demonstrates how to use RustCloud's `LlmProvider` implementation for Azure OpenAI Service, providing access to OpenAI foundation models (GPT-4o, GPT-4, GPT-3.5-Turbo, and embeddings) through your Azure resource.

## Prerequisites

1. **Azure Account** with an Azure OpenAI resource created
2. **Model deployments** configured in Azure OpenAI Studio

### Create a Deployment

Go to [Azure OpenAI Studio](https://oai.azure.com) → Deployments → Create new deployment. Choose a model (e.g., `gpt-4o`) and give it a deployment name — this is the name you pass to RustCloud.

## Credential Setup

**Environment variables:**
```bash
export AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com
export AZURE_OPENAI_API_KEY=your-api-key
export AZURE_OPENAI_API_VERSION=2024-10-21        # optional, defaults to 2024-10-21
export AZURE_OPENAI_EMBED_DEPLOYMENT=text-embedding-ada-002  # optional, defaults shown
```

Your endpoint and API key are available in the Azure Portal under your OpenAI resource → Keys and Endpoint.

## Basic Usage

### Text Generation

Generate text using any deployed model:

```rust
use rustcloud::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = AzureOpenAIProvider::new();

    let request = LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Explain quantum computing in 100 words".to_string(),
        }],
        max_tokens: Some(150),
        temperature: Some(0.7),
        system_prompt: Some("You are a physics expert.".to_string()),
    };

    let response = provider.generate(request).await?;
    println!("Response: {}", response.text);
    println!("Finish reason: {:?}", response.finish_reason);

    if let Some(usage) = response.usage {
        println!("Tokens used — prompt: {}, completion: {}",
            usage.prompt_tokens, usage.completion_tokens);
    }

    Ok(())
}
```

### Streaming Generation

Stream responses token-by-token for real-time output:

```rust
use rustcloud::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef, LlmStreamEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = AzureOpenAIProvider::new();

    let request = LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Write a short story about a robot learning to paint.".to_string(),
        }],
        max_tokens: Some(400),
        temperature: Some(0.8),
        system_prompt: None,
    };

    let mut stream = provider.stream(request).await?;

    while let Some(event) = stream.next().await {
        match event {
            LlmStreamEvent::DeltaText(text) => print!("{}", text),
            LlmStreamEvent::Usage(stats) => {
                println!("\nPrompt tokens: {}, Completion tokens: {}",
                    stats.prompt_tokens, stats.completion_tokens);
            }
            LlmStreamEvent::Done(reason) => println!("\nStream ended: {:?}", reason),
            LlmStreamEvent::Error(e) => eprintln!("Stream error: {}", e),
        }
    }

    Ok(())
}
```

### Text Embeddings

Generate embeddings for multiple texts in a single API call:

```rust
use rustcloud::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;
use rustcloud::traits::llm_provider::LlmProvider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = AzureOpenAIProvider::new();

    let texts = vec![
        "The cat sat on the mat".to_string(),
        "A feline rested on the rug".to_string(),
        "The dog ran in the park".to_string(),
    ];

    let response = provider.embed(texts).await?;

    for (i, embedding) in response.embeddings.iter().enumerate() {
        println!("Text {} embedding dimension: {}", i, embedding.len());
    }

    Ok(())
}
```

Embeddings use the deployment set in `AZURE_OPENAI_EMBED_DEPLOYMENT` (defaults to `text-embedding-ada-002`). All texts are sent in a single batched request.

### Function Calling (Tool Use)

Use structured tool interactions to let the model call your functions:

```rust
use rustcloud::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef, ToolDefinition, ToolCallResponse};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = AzureOpenAIProvider::new();

    let tools = vec![
        ToolDefinition {
            name: "get_weather".to_string(),
            description: "Get current weather for a location".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "City name"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"]
                    }
                },
                "required": ["location"]
            }),
        },
    ];

    let request = LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What's the weather like in Seattle?".to_string(),
        }],
        max_tokens: Some(256),
        temperature: Some(0.0),
        system_prompt: None,
    };

    let response = provider.generate_with_tools(request, tools).await?;

    match response {
        ToolCallResponse::Text(llm_resp) => println!("Text response: {}", llm_resp.text),
        ToolCallResponse::ToolCall { name, arguments } => {
            println!("Tool called: {}", name);
            println!("Arguments: {}", arguments);
        }
    }

    Ok(())
}
```

### Bring Your Own Config

If you already have credentials ready:

```rust
use rustcloud::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;

fn main() {
    let provider = AzureOpenAIProvider::with_config(
        "https://your-resource.openai.azure.com",
        "your-api-key",
        "2024-10-21",
        "text-embedding-ada-002",
    );
    // use provider...
}
```

## ModelRef — Deployment vs Provider

Azure OpenAI uses **deployment names**, not model names directly. Both `ModelRef::Deployment` and `ModelRef::Provider` are treated as the deployment ID:

```rust
// Explicit deployment reference (preferred for Azure)
ModelRef::Deployment("my-gpt4o-prod".to_string())

// Provider reference — treated as deployment ID
ModelRef::Provider("gpt-4o".to_string())  // only works if deployment name = model name
```

`ModelRef::Logical` is not supported for Azure OpenAI and returns an error.

## Available Models

### Chat / Completion

| Deployment Model | Description |
|---|---|
| `gpt-4o` | Latest GPT-4o — fast, multimodal, cost-effective |
| `gpt-4o-mini` | Smaller GPT-4o — lowest cost |
| `gpt-4` | GPT-4 Turbo — high capability |
| `gpt-35-turbo` | GPT-3.5 Turbo — fast and affordable |

### Embeddings

| Deployment Model | Dimensions | Description |
|---|---|---|
| `text-embedding-ada-002` | 1536 | Standard embedding model |
| `text-embedding-3-small` | 1536 | Newer, more efficient |
| `text-embedding-3-large` | 3072 | Highest quality |

> Embeddings always use the deployment set in `AZURE_OPENAI_EMBED_DEPLOYMENT`, regardless of the model in `LlmRequest`.

## Supported Operations Matrix

| Operation | GPT-4o | GPT-4 | GPT-3.5-Turbo | Embedding models |
|---|---|---|---|---|
| generate | ✅ | ✅ | ✅ | ❌ |
| stream | ✅ | ✅ | ✅ | ❌ |
| embed | ✅ (separate deployment) | ✅ | ✅ | ✅ |
| generate_with_tools | ✅ | ✅ | ✅ | ❌ |

## Configuration Options

| Parameter | Type | Description |
|---|---|---|
| `max_tokens` | `Option<u32>` | Maximum output tokens |
| `temperature` | `Option<f32>` | Randomness (0.0–1.0) |
| `system_prompt` | `Option<String>` | System instruction prepended to messages |
| `model` | `ModelRef` | Deployment or Provider reference |

## Error Handling

```rust
match provider.generate(request).await {
    Ok(response) => println!("Success: {}", response.text),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Troubleshooting

### Authentication Failed
```
Error: provider error 401: Access denied due to invalid subscription key
```
- Verify `AZURE_OPENAI_API_KEY` matches the key shown in Azure Portal → Resource → Keys and Endpoint
- Ensure you're using the API key, not the Azure AD token (this provider uses key auth)

### Deployment Not Found
```
Error: provider error 404: The API deployment for this resource does not exist
```
- Check the deployment name in Azure OpenAI Studio → Deployments
- Deployment names are case-sensitive
- Verify `AZURE_OPENAI_ENDPOINT` matches your resource's endpoint URL exactly

### Rate Limit Exceeded
```
Error: provider error 429: Requests to the ChatCompletions operation have exceeded call rate limit
```
- Implement retry with exponential backoff
- Request a quota increase via Azure Support
- Consider using a different region or deployment

### Region Availability
Not all models are available in all Azure regions. Check [Azure OpenAI model availability](https://learn.microsoft.com/en-us/azure/ai-services/openai/concepts/models) for the current matrix.

## References

- [Azure OpenAI REST API Reference](https://learn.microsoft.com/en-us/azure/ai-services/openai/reference)
- [Chat Completions Guide](https://learn.microsoft.com/en-us/azure/ai-services/openai/how-to/chatgpt)
- [Function Calling](https://learn.microsoft.com/en-us/azure/ai-services/openai/how-to/function-calling)
- [Embeddings Guide](https://learn.microsoft.com/en-us/azure/ai-services/openai/how-to/embeddings)
- [Azure OpenAI Studio](https://oai.azure.com)
