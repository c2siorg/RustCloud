# RustCloud

RustCloud is a Rust library that hides the differences between APIs provided by various cloud providers (AWS, GCP, Azure, and more), letting you manage cloud resources through a single, consistent interface.

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org)
[![Slack](https://img.shields.io/badge/chat-slack-purple.svg)](https://c2si.slack.com/archives/rust-cloud)

> **Note:** This is the Rust port of the original [gocloud](https://github.com/cloudlibz/gocloud) library. The install instructions below are for Rust/Cargo — ignore any Go references you may see in older branches.

---

## Table of Contents

- [Overview](#overview)
- [Service Types](#service-types)
- [Supported Providers](#supported-providers)
- [Getting Started](#getting-started)
- [Credential Setup](#credential-setup)
- [Usage](#usage)
- [LLM Provider Abstraction](#llm-provider-abstraction)
- [Development](#development)
- [Running Tests](#running-tests)
- [Contributing](#contributing)

---

## Overview

The core idea is straightforward: you should be able to switch between AWS and GCP (or add a new provider entirely) without rewriting your application logic. RustCloud defines traits for each service category, and each cloud provider implements those traits.

```
Your application code
        │
        ▼
┌──────────────────────┐
│   RustCloud Traits   │  ← unified API surface
└──────────┬───────────┘
           │
    ┌──────┼──────┐
    ▼      ▼      ▼
  AWS     GCP   Azure   ...
```

All I/O is async (backed by Tokio), and errors are returned as the `CloudError` enum so you can match on them precisely.

---

## Service Types

| Type | Description |
|---|---|
| **Compute** | Manage virtual machines and cloud servers |
| **Container** | Deploy and manage containerized workloads |
| **Database** | Interact with managed database services |
| **Storage** | Object storage, block storage, and archival |
| **Network** | Load balancers and DNS management |
| **Security** | Identity, access management, and key management |
| **AI/ML** | Machine learning and LLM provider abstractions |

---

## Supported Providers

### AWS

| Category | Service |
|---|---|
| Compute | EC2, ECS, EKS |
| Database | DynamoDB |
| Management | CloudWatch |
| Network | Route53, Elastic Load Balancing |
| Security | IAM, KMS |
| Storage | S3, Glacier, Block Storage |

Examples: [`examples/aws/`](examples/aws/)
| Compute | [EC2](examples/aws/compute/ec2.md), [ECS](examples/aws/compute/ecs.md), [EKS](examples/aws/compute/eks.md) |
| Database | [DynamoDB](examples/aws/database/dynamodb.md) |
| Management | [CloudWatch](examples/aws/management/monitoring.md) |
| Network | [Route53](examples/aws/network/dns.md), [Elastic Load Balancing](examples/aws/network/loadbalancer.md) |
| Security | [IAM](examples/aws/security/iam.md), [KMS](examples/aws/security/kms.md), [Secrets Manager](examples/aws/security/secrets_manager.md) |
| Storage | [S3](examples/aws/storage/bucket.md), [Glacier](examples/aws/storage/archival.md), [Block Storage](examples/aws/storage/block.md) |
| App Services | [SNS](examples/aws/app_services/sns.md), [SQS](examples/aws/app_services/sqs.md) |
| AI / ML | [Bedrock](examples/aws/artificial_intelligence/bedrock.md) |

### Google Cloud Platform

| Category | Service |
|---|---|
| AI / ML | AutoML |
| App Services | Cloud Pub/Sub |
| Compute | Compute Engine, GKE |
| Database | Bigtable, BigQuery |
| Network | Cloud DNS, Load Balancing |
| Storage | Cloud Storage |

Examples: [`examples/gcp/`](examples/gcp/)
| AI / ML | [AutoML](examples/gcp/artificial_intelligence/automl.md), [Vertex AI](examples/gcp/artificial_intelligence/vertex_ai.md) |
| App Services | [Cloud Pub/Sub](examples/gcp/app_services/notifications.md) |
| Compute | [Compute Engine](examples/gcp/compute/compute_engine.md), [GKE](examples/gcp/compute/kubernetes.md) |
| Database | [Bigtable](examples/gcp/database/bigtable.md), BigQuery |
| Network | [Cloud DNS](examples/gcp/network/dns.md), [Load Balancing](examples/gcp/network/loadbalancer.md) |
| Storage | [Cloud Storage (disks)](examples/gcp/storage/storage.md), [Cloud Storage (GCS)](examples/gcp/storage/cloud_storage.md) |

### Azure *(in progress)*

| Category | Service |
|---|---|
| Auth | Azure authentication |
| Security | [Key Vault](examples/azure/security/key_vault.md) |
| Storage | Blob Storage |

### DigitalOcean *(in progress)*

| Category | Service |
|---|---|
| Compute | Droplets |
| Network | Load Balancer |
| DNS | DigitalOcean DNS |
| Storage | Block Storage |

---

## Getting Started

### Prerequisites

You need a working Rust toolchain. If you don't have one:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This installs `rustup`, `cargo`, and the stable Rust compiler. The project requires Rust 2021 edition.

### Add to your project

RustCloud is not yet published to crates.io, so reference it directly from the repository:

```toml
[dependencies]
rustcloud = { git = "https://github.com/c2siorg/RustCloud", subdirectory = "rustcloud" }
tokio = { version = "1", features = ["full"] }
```

### Clone and build

```sh
git clone https://github.com/c2siorg/RustCloud
cd RustCloud/rustcloud
cargo build
```

---

## Credential Setup

RustCloud uses the standard credential mechanisms for each provider, so you don't need any custom config file format.

### AWS

The AWS SDK for Rust uses the same credential chain as the AWS CLI. The easiest options are environment variables or the shared credentials file.

**Environment variables:**
```sh
export AWS_ACCESS_KEY_ID="your-key-id"
export AWS_SECRET_ACCESS_KEY="your-secret-key"
export AWS_DEFAULT_REGION="us-east-1"
```

**Credentials file** at `~/.aws/credentials`:
```ini
[default]
aws_access_key_id = your-key-id
aws_secret_access_key = your-secret-key
```

### Google Cloud Platform

GCP uses Application Default Credentials (ADC). Point the environment variable at your service account key file:

```sh
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/service-account-key.json"
```

You can download a service account key from the [GCP Console](https://console.cloud.google.com/iam-admin/serviceaccounts). If you're running locally with the `gcloud` CLI installed, `gcloud auth application-default login` works too.

### Azure

```sh
export AZURE_CLIENT_ID="your-client-id"
export AZURE_CLIENT_SECRET="your-client-secret"
export AZURE_TENANT_ID="your-tenant-id"
```

### DigitalOcean

```sh
export DIGITALOCEAN_TOKEN="your-token"
```

---

## Usage

All operations are async and return `Result<_, CloudError>`. A minimal example using the AWS EC2 module:

```rust
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::Client;
use rustcloud::aws::aws_apis::compute::aws_ec2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let region = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region).load().await;
    let client = Client::new(&config);

    let instance_id = aws_ec2::create_instance(&client, "ami-0abcdef1234567890").await?;
    println!("Created instance: {}", instance_id);

    Ok(())
}
```

For more complete examples, see the [`examples/`](examples/) directory — each service has its own markdown file with copy-pasteable snippets.

---

## LLM Provider Abstraction

One of the newer additions to RustCloud is a unified interface for interacting with large language model (LLM) providers. The goal is the same as the rest of the library: write your AI code once, swap the backend provider without touching application logic.

### The `LlmProvider` trait

```rust
use rustcloud::traits::llm_provider::LlmProvider;
use rustcloud::types::llm::{LlmRequest, ModelRef, Message};

// Any provider that implements LlmProvider can be used here
async fn ask(provider: &dyn LlmProvider, question: &str) {
    let req = LlmRequest {
        model: ModelRef::Logical {
            family: "gemini".to_string(),
            tier: Some("pro".to_string()),
        },
        messages: vec![Message {
            role: "user".to_string(),
            content: question.to_string(),
        }],
        max_tokens: Some(512),
        temperature: Some(0.7),
        system_prompt: None,
    };

    let response = provider.generate(req).await.unwrap();
    println!("{}", response.text);
}
```

### Available operations

| Method | Description |
|---|---|
| `generate` | Standard text generation (request/response) |
| `stream` | Streaming generation via async `Stream` |
| `embed` | Get embeddings for a list of texts |
| `generate_with_tools` | Text generation with tool/function calling |

### `ModelRef` — flexible model targeting

Instead of embedding provider-specific model IDs everywhere, `ModelRef` gives you three options:

```rust
// Reference a specific provider model ID (e.g., for Bedrock, Vertex AI)
ModelRef::Provider("anthropic.claude-3-sonnet-20240229-v1:0".to_string())

// Reference a model logically — the provider implementation resolves this
ModelRef::Logical { family: "claude".to_string(), tier: Some("sonnet".to_string()) }

// Reference a named deployment (e.g., Azure OpenAI deployments)
ModelRef::Deployment("my-gpt4-deployment".to_string())
```

This abstraction is what makes it practical to target Vertex AI, AWS Bedrock, and Azure OpenAI from the same calling code.

> The GSoC 2026 project is extending this by adding concrete implementations for **BigQuery**, **Vertex AI**, **AWS Bedrock GenAI**, and **Azure OpenAI**. If you're interested in contributing, see [issue #36](https://github.com/c2siorg/RustCloud/issues/36) and the `#rust-cloud` Slack channel.

---

## Development

```sh
git clone https://github.com/c2siorg/RustCloud
cd RustCloud/rustcloud
cargo build
```

Before submitting a PR, run the formatter and linter:

```sh
cargo fmt
cargo clippy -- -D warnings
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full contribution guide.

---

## Running Tests

```sh
cd RustCloud/rustcloud
cargo test
```

To run tests for a specific provider:

```sh
cargo test aws      # all AWS tests
cargo test gcp      # all GCP tests
```

**Important:** Tests that create real cloud resources will create live infrastructure. Make sure you clean up any instances, storage buckets, load balancers, and DNS records after running integration tests — check each provider's console.

> **GCP note:** Some GCP tests currently have a known compilation issue with struct initialization (see [#14](https://github.com/c2siorg/RustCloud/issues/14)). Unit tests and AWS tests compile and run correctly.

---

## Contributing

Contributions are welcome. A few things to keep in mind:

- Comment on an issue before starting work — it avoids duplicate effort
- Keep PRs focused; one logical change per PR is easier to review
- Run `cargo fmt` and `cargo clippy` before pushing
- Add tests for new functionality

For details, see [CONTRIBUTING.md](CONTRIBUTING.md). To discuss ideas or ask questions, join the `#rust-cloud` channel on [c2si.slack.com](https://c2si.slack.com).

---

## License

Apache 2.0 — see [LICENSE](LICENSE).
