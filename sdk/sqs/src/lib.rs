#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! <p>Welcome to the <i>Amazon SQS API Reference</i>.</p>
//! <p>Amazon SQS is a reliable, highly-scalable hosted queue for storing messages as they travel between applications or microservices. Amazon SQS moves data between distributed application components and helps you decouple these components.</p>
//! <p>For information on the permissions you need to use this API, see
//! <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-authentication-and-access-control.html">Identity and
//! access management</a> in the <i>Amazon SQS Developer Guide.</i>
//! </p>  
//! <p>You can use <a href="http://aws.amazon.com/tools/#sdk">Amazon Web Services SDKs</a> to access Amazon SQS using your favorite programming language. The SDKs perform tasks such as the following automatically:</p>
//! <ul>
//! <li>
//! <p>Cryptographically sign your service requests</p>
//! </li>
//! <li>
//! <p>Retry requests</p>
//! </li>
//! <li>
//! <p>Handle error responses</p>
//! </li>
//! </ul>
//! <p>
//! <b>Additional information</b>
//! </p>
//! <ul>
//! <li>
//! <p>
//! <a href="http://aws.amazon.com/sqs/">Amazon SQS Product Page</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <i>Amazon SQS Developer Guide</i>
//! </p>
//! <ul>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-making-api-requests.html">Making API Requests</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-metadata.html#sqs-message-attributes">Amazon SQS Message Attributes</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">Amazon SQS Dead-Letter Queues</a>
//! </p>
//! </li>
//! </ul>
//! </li>
//! <li>
//! <p>
//! <a href="http://docs.aws.amazon.com/cli/latest/reference/sqs/index.html">Amazon SQS in the <i>Command Line Interface</i>
//! </a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <i>Amazon Web Services General Reference</i>
//! </p>
//! <ul>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html#sqs_region">Regions and Endpoints</a>
//! </p>
//! </li>
//! </ul>
//! </li>
//! </ul>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
mod query_ser;
mod rest_xml_wrapped_errors;
mod xml_deser;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("sqs", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
