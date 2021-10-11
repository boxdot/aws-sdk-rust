#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! <fullname>IoT data</fullname>
//! <p>IoT data enables secure, bi-directional communication between Internet-connected things (such as sensors,
//! actuators, embedded devices, or smart appliances) and the Amazon Web Services cloud. It implements a broker for applications and
//! things to publish messages over HTTP (Publish) and retrieve, update, and delete shadows. A shadow is a
//! persistent representation of your things and their state in the Amazon Web Services cloud.</p>
//! <p>Find the endpoint address for actions in IoT data by running this CLI command:</p>
//! <p>
//! <code>aws iot describe-endpoint --endpoint-type iot:Data-ATS</code>
//! </p>
//! <p>The service name used by <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Amazon Web ServicesSignature Version 4</a>
//! to sign requests is: <i>iotdevicegateway</i>.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
mod http_serde;
pub mod input;
mod json_deser;
mod json_errors;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("iotdataplane", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
