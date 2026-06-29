
# rustcloud - AWS Bedrock

## Configure AWS credentials

```sh
export AWS_ACCESS_KEY_ID="xxxxxxxxxxxx"
export AWS_SECRET_ACCESS_KEY="xxxxxxxxxxxx"
export AWS_REGION="us-east-1"
```

Alternatively, configure credentials via `~/.aws/credentials` or an IAM role attached to your instance.

## Initialize the provider

```rust
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;

#[tokio::main]
async fn main() {
    let provider = BedrockProvider::new().await;
}
```

## Generate text

```rust
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef};

#[tokio::main]
async fn main() {
    let provider = BedrockProvider::new().await;

    let req = LlmRequest {
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
        messages: vec![Message { role: "user".to_string(), content: "What is 2 + 2?".to_string() }],
        max_tokens: Some(256),
        temperature: Some(0.7),
        system_prompt: Some("You are a helpful assistant.".to_string()),
    };

    let response = provider.generate(req).await.unwrap();
    println!("{}", response.text);
}
```

## Stream a response

```rust
use futures::StreamExt;
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, LlmStreamEvent, Message, ModelRef};

#[tokio::main]
async fn main() {
    let provider = BedrockProvider::new().await;

    let req = LlmRequest {
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Explain async/await in Rust.".to_string(),
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

## Generate embeddings

```rust
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
use rustcloud::traits::llm_provider::LlmProvider;

#[tokio::main]
async fn main() {
    let provider = BedrockProvider::new().await;

    let texts = vec![
        "The quick brown fox jumps over the lazy dog.".to_string(),
        "Rust is a systems programming language focused on safety.".to_string(),
    ];

    let response = provider.embed(texts).await.unwrap();

    for (i, embedding) in response.embeddings.iter().enumerate() {
        println!("embedding {}: {} dimensions", i, embedding.len());
    }
}
```

## Generate with tools

```rust
use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, Message, ModelRef, ToolCallResponse, ToolDefinition};

#[tokio::main]
async fn main() {
    let provider = BedrockProvider::new().await;

    let req = LlmRequest {
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What is the weather in London?".to_string(),
        }],
        max_tokens: Some(256),
        temperature: None,
        system_prompt: None,
    };

    let tools = vec![ToolDefinition {
        name: "get_weather".to_string(),
        description: "Returns current weather for a given city.".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "city": { "type": "string", "description": "City name" }
            },
            "required": ["city"]
        }),
    }];

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
