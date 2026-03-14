# GCP Vertex AI (Gemini) Integration

This guide demonstrates how to use RustCloud's `LlmProvider` implementation for GCP Vertex AI, powered by Gemini models.

## Prerequisites

1. **GCP Project**: Create a GCP project with Vertex AI API enabled
2. **Authentication**: Set up gcloud authentication on your machine

```bash
gcloud auth login
gcloud config set project YOUR_PROJECT_ID
```

3. **Environment Variables**:

```bash
export GOOGLE_CLOUD_PROJECT=your-project-id
export GOOGLE_APPLICATION_CREDENTIALS=/path/to/your/service-account-key.json
```

## Basic Usage

### Text Generation

Generate text using Gemini models:

```rust
use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAI;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vertex_ai = VertexAI::new("your-project-id", "us-central1");

    let request = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Explain quantum computing in 100 words".to_string(),
        }],
        max_tokens: Some(150),
        temperature: Some(0.7),
        system_prompt: Some("You are a physics expert.".to_string()),
    };

    let response = vertex_ai.generate(request).await?;
    println!("Response: {}", response.text);

    Ok(())
}
```

### Streaming Generation

Stream responses for long-form content:

```rust
use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAI;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef, LlmStreamEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vertex_ai = VertexAI::new("your-project-id", "us-central1");

    let request = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-pro".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Write a 500-word essay on AI ethics.".to_string(),
        }],
        max_tokens: Some(600),
        temperature: Some(0.8),
        system_prompt: None,
    };

    let mut stream = vertex_ai.stream(request).await?;

    while let Some(event) = stream.next().await {
        match event {
            LlmStreamEvent::DeltaText(text) => print!("{}", text),
            LlmStreamEvent::Usage(stats) => {
                println!("\nPrompt tokens: {}, Completion tokens: {}",
                    stats.prompt_tokens, stats.completion_tokens);
            }
            LlmStreamEvent::Done(reason) => println!("Stream ended: {:?}", reason),
            LlmStreamEvent::Error(e) => eprintln!("Stream error: {}", e),
        }
    }

    Ok(())
}
```

### Text Embeddings

Generate embeddings for semantic search and similarity:

```rust
use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAI;
use rustcloud::traits::llm_provider::LlmProvider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vertex_ai = VertexAI::new("your-project-id", "us-central1");

    let texts = vec![
        "The cat sat on the mat".to_string(),
        "A feline rested on the rug".to_string(),
        "The dog ran in the park".to_string(),
    ];

    let embeddings = vertex_ai.embed(texts).await?;

    for (i, embedding) in embeddings.embeddings.iter().enumerate() {
        println!("Text {} embedding dimension: {}", i, embedding.len());
    }

    Ok(())
}
```

### Function Calling (Tool Use)

Use Gemini's function calling to enable structured interactions:

```rust
use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAI;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef, ToolDefinition, ToolCallResponse};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vertex_ai = VertexAI::new("your-project-id", "us-central1");

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
        ToolDefinition {
            name: "calculate".to_string(),
            description: "Perform mathematical calculations".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "Mathematical expression"
                    }
                },
                "required": ["expression"]
            }),
        },
    ];

    let request = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-pro".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What's the weather like in New York? Also calculate 25 * 4".to_string(),
        }],
        max_tokens: Some(256),
        temperature: Some(0.0),
        system_prompt: None,
    };

    let response = vertex_ai.generate_with_tools(request, tools).await?;

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

## Available Models

### Generative Models
- `gemini-1.5-flash` - Fast, cost-effective model for simple tasks
- `gemini-1.5-pro` - Advanced model for complex reasoning
- `gemini-2.0-flash` - Latest generation (when available)

### Embedding Models
- `text-embedding-004` - Default embedding model
- `text-embedding-preview-0409` - Preview version

## Configuration Options

### Temperature
Controls response randomness (0.0 - 1.0):
- `0.0`: Deterministic, good for structured tasks
- `0.5`: Balanced creativity and consistency
- `1.0`: Maximum randomness and creativity

### Max Tokens
Limit response length. Example values:
- `100`: Short responses
- `500`: Medium responses
- `2000`: Longer, detailed responses

### System Prompt
Define model behavior and context:
```rust
system_prompt: Some("You are a Shakespearean writing assistant. Respond in iambic pentameter.".to_string())
```

## Error Handling

Always handle potential errors from authentication and API calls:

```rust
match vertex_ai.generate(request).await {
    Ok(response) => println!("Success: {}", response.text),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Rate Limits and Quotas

Be aware of Vertex AI rate limits:
- Check GCP Console > Vertex AI > API usage dashboard
- Request quota increases if needed
- Implement exponential backoff for retries

## Cost Optimization

1. Use `gemini-1.5-flash` for development and simple tasks
2. Use `gemini-1.5-pro` only for complex reasoning needs
3. Monitor token usage in response metadata
4. Batch multiple requests when possible
5. Cache common prompts and responses

## Troubleshooting

### Authentication Errors
```
Error: Auth failed
```
- Verify gcloud is properly configured: `gcloud auth application-default print-access-token`
- Check service account has Vertex AI permissions
- Ensure `GOOGLE_APPLICATION_CREDENTIALS` is set correctly

### Model Not Found
```
Error: 404 Not Found
```
- Verify location supports the model (e.g., some models only in us-central1)
- Check model name spelling

### Rate Limit Exceeded
```
Error: 429 Too Many Requests
```
- Implement exponential backoff
- Reduce request frequency
- Request quota increase from GCP

## References

- [Vertex AI Docs](https://cloud.google.com/vertex-ai/docs)
- [Gemini API Reference](https://ai.google.dev/api/rest)
- [Model Card: Gemini 1.5](https://ai.google.dev/models/gemini-1-5)
