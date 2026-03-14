//! # RustCloud
//!
//! A multi-cloud SDK for Rust, providing unified access to
//! AWS, Azure, GCP, and DigitalOcean cloud APIs.

pub mod errors;
pub mod types;
pub mod traits;
pub mod aws;
pub mod azure;
pub mod gcp;
pub mod digiocean;

pub use errors::CloudError;

#[cfg(test)]
mod tests;
