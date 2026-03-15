//! # RustCloud
//!
//! A multi-cloud SDK for Rust, providing unified access to
//! AWS, Azure, GCP, and DigitalOcean cloud APIs.

// Allow design-level lints that are baked into the existing API surface.
// These can be addressed in future refactoring issues.
#![allow(clippy::result_large_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(non_snake_case)]
#![allow(deprecated)]

pub mod aws;
pub mod azure;
pub mod digiocean;
pub mod errors;
pub mod gcp;
pub mod traits;
pub mod types;

pub use errors::CloudError;

#[cfg(test)]
mod tests;
