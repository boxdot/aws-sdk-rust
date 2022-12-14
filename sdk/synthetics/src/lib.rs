#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Amazon CloudWatch Synthetics</fullname>
//! <p>You can use Amazon CloudWatch Synthetics to continually monitor your services. You can
//! create and manage <i>canaries</i>, which are modular, lightweight scripts
//! that monitor your endpoints and APIs
//! from the outside-in. You can set up your canaries to run
//! 24 hours a day, once per minute. The canaries help you check the availability and latency
//! of your web services and troubleshoot anomalies by investigating load time data,
//! screenshots of the UI, logs, and metrics. The canaries seamlessly integrate with CloudWatch
//! ServiceLens to help you trace the causes of impacted nodes in your applications. For more
//! information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/ServiceLens.html">Using ServiceLens to Monitor
//! the Health of Your Applications</a> in the <i>Amazon CloudWatch User
//! Guide</i>.</p>
//!
//! <p>Before you create and manage canaries, be aware of the security considerations. For more
//! information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/servicelens_canaries_security.html">Security
//! Considerations for Synthetics Canaries</a>.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// All error types that operations can return.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
pub mod lens;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Paginators for the service
pub mod paginator;
/// Data primitives referenced by other data types.
pub mod types;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("synthetics", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
