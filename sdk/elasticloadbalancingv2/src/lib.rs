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
//! <fullname>Elastic Load Balancing</fullname>
//!
//! <p>A load balancer distributes incoming traffic across targets, such as your EC2 instances.
//! This enables you to increase the availability of your application. The load balancer also
//! monitors the health of its registered targets and ensures that it routes traffic only to
//! healthy targets. You configure your load balancer to accept incoming traffic by specifying one
//! or more listeners, which are configured with a protocol and port number for connections from
//! clients to the load balancer. You configure a target group with a protocol and port number for
//! connections from the load balancer to the targets, and with health check settings to be used
//! when checking the health status of the targets.</p>
//!
//! <p>Elastic Load Balancing supports the following types of load balancers: Application Load
//! Balancers, Network Load Balancers, Gateway Load Balancers, and Classic Load Balancers. This
//! reference covers the following load balancer types:</p>
//! <ul>
//! <li>
//! <p>Application Load Balancer - Operates at the application layer (layer 7) and supports
//! HTTP and HTTPS.</p>
//! </li>
//! <li>
//! <p>Network Load Balancer - Operates at the transport layer (layer 4) and supports TCP,
//! TLS, and UDP.</p>
//! </li>
//! <li>
//! <p>Gateway Load Balancer - Operates at the network layer (layer 3).</p>
//! </li>
//! </ul>
//!
//! <p>For more information, see the <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/">Elastic Load Balancing User
//! Guide</a>.</p>
//!
//!
//!
//!
//!
//!
//!
//! <p>All Elastic Load Balancing operations are idempotent, which means that they complete at
//! most one time. If you repeat an operation, it succeeds.</p>
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
mod query_ser;
mod rest_xml_wrapped_errors;
/// Data primitives referenced by other data types.
pub mod types;
mod xml_deser;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("elasticloadbalancingv2", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
