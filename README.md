# RustCloud
RustCloud is a Rust library that provides a unified API across cloud providers so users can manage resources with a consistent interface.

## Current Scope
This repository currently contains APIs and tests focused on:

- AWS: compute, storage, networking, security, monitoring, database
- GCP: compute, storage, networking, app services, AI, database

## Repository Layout
- `rustcloud/`: main Rust crate
- `rustcloud/src/aws/`: AWS API modules
- `rustcloud/src/gcp/`: GCP API modules
- `rustcloud/src/tests/`: integration-style tests
- `examples/`: provider/service usage notes and examples

## Quickstart
1. Clone the repository:
```bash
git clone https://github.com/c2siorg/RustCloud.git
cd RustCloud
```

2. Verify project builds from repository root:
```bash
cargo check
```

3. Run formatting and lint checks:
```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

4. Run tests:
```bash
cargo test -p rustcloud
```

## Examples
Examples are organized by provider and service under `examples/`.

- AWS:
  - `examples/aws/compute/ec2.md`
  - `examples/aws/compute/ecs.md`
  - `examples/aws/compute/eks.md`
  - `examples/aws/storage/bucket.md`
  - `examples/aws/storage/block.md`
  - `examples/aws/storage/archival.md`
  - `examples/aws/network/dns.md`
  - `examples/aws/network/loadbalancer.md`
  - `examples/aws/security/iam.md`
  - `examples/aws/security/kms.md`
  - `examples/aws/database/dynamodb.md`
  - `examples/aws/management/monitoring.md`

- GCP:
  - `examples/gcp/compute/compute_engine.md`
  - `examples/gcp/compute/kubenetes.md`
  - `examples/gcp/storage/storage.md`
  - `examples/gcp/network/dns.md`
  - `examples/gcp/network/loadbalancer.md`
  - `examples/gcp/database/bigtable.md`
  - `examples/gcp/artificial_intelligence/automl.md`
  - `examples/gcp/app_services/notifications.md`

## Contributing
Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening issues or pull requests.

## Notes for Integration Tests
Some tests interact with real cloud APIs and may require valid credentials and cloud-side cleanup.
