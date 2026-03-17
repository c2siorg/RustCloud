# AWS Bedrock Integration

This guide demonstrates how to use RustCloud's `LlmProvider` implementation for AWS Bedrock, providing access to foundation models including Anthropic Claude, Amazon Titan, Meta Llama, and Mistral.

## Prerequisites

1. **AWS Account** with Bedrock model access enabled
2. **Enable models** in the AWS Console under Bedrock > Model Access

### Enable Model Access
Go to [AWS Bedrock console](https://console.aws.amazon.com/bedrock) → Model Access → Request access for the models you want to use.

## Credential Setup

**Environment variables:**
```bash
export AWS_ACCESS_KEY_ID=your-access-key
export AWS_SECRET_ACCESS_KEY=your-secret-key
export AWS_DEFAULT_REGION=us-east-1
```

**Or credentials file** at `~/.aws/credentials`:
```ini
[default]
aws_access_key_id = your-access-key
aws_secret_access_key = your-secret-key
```

## Basic Usage

### Text Generation

Generate text using any Bedrock foundation model:

```rust
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = BedrockProvider::new().await;

    let request = LlmRequest {
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
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
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef, LlmStreamEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = BedrockProvider::new().await;

    let request = LlmRequest {
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
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

Generate embeddings using Amazon Titan Embed Text (used automatically):

```rust
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
use rustcloud::traits::llm_provider::LlmProvider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = BedrockProvider::new().await;

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

### Function Calling (Tool Use)

Use Bedrock's Converse API for structured tool interactions:

```rust
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef, ToolDefinition, ToolCallResponse};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = BedrockProvider::new().await;

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
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
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

### Bring Your Own Client

If you already have an AWS config set up:

```rust
use aws_sdk_bedrockruntime::Client;
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let provider = BedrockProvider::with_client(client);
    // use provider...
}
```

## Available Models

### Anthropic Claude (Recommended)
- `anthropic.claude-3-5-haiku-20241022-v1:0` — Fast, cost-effective
- `anthropic.claude-3-5-sonnet-20241022-v2:0` — Balanced performance
- `anthropic.claude-3-opus-20240229-v1:0` — Most capable

### Amazon Titan
- `amazon.titan-text-express-v1` — Fast text generation
- `amazon.titan-text-premier-v1:0` — Premium quality
- `amazon.titan-embed-text-v2:0` — Embeddings (used automatically)

### Meta Llama
- `meta.llama3-2-3b-instruct-v1:0` — Lightweight
- `meta.llama3-2-90b-instruct-v1:0` — High performance

### Mistral
- `mistral.mistral-7b-instruct-v0:2` — Efficient
- `mistral.mixtral-8x7b-instruct-v0:1` — Mixture of experts

## Configuration Options

| Parameter | Type | Description |
|---|---|---|
| `max_tokens` | `Option<u32>` | Maximum output tokens |
| `temperature` | `Option<f32>` | Randomness (0.0–1.0) |
| `system_prompt` | `Option<String>` | System instruction |
| `model` | `ModelRef` | Model to use |

## Supported Operations Matrix

| Operation | Claude | Titan Text | Llama | Mistral |
|---|---|---|---|---|
| generate | ✅ | ✅ | ✅ | ✅ |
| stream | ✅ | ✅ | ✅ | ✅ |
| embed | ✅ (Titan Embed) | ✅ | ✅ (Titan Embed) | ✅ (Titan Embed) |
| generate_with_tools | ✅ | ❌ | ✅ | ✅ |

> **Note:** Embeddings always use `amazon.titan-embed-text-v2:0` regardless of the model specified in the struct, as it is the standard embedding model on Bedrock.

## Error Handling

```rust
match provider.generate(request).await {
    Ok(response) => println!("Success: {}", response.text),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Troubleshooting

### Model Access Denied
```
Error: AccessDeniedException
```
- Enable the model in AWS Console → Bedrock → Model Access
- Ensure IAM policy includes `bedrock:InvokeModel` and `bedrock:Converse`

### Region Not Supported
```
Error: ResourceNotFoundException
```
- Not all models are available in all regions
- Switch to `us-east-1` or `us-west-2` for widest model support

### Throttling
```
Error: ThrottlingException
```
- Implement retry with exponential backoff
- Request a quota increase via AWS Support

## References

- [AWS Bedrock Docs](https://docs.aws.amazon.com/bedrock)
- [Converse API Reference](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_Converse.html)
- [Supported Models](https://docs.aws.amazon.com/bedrock/latest/userguide/models-supported.html)
