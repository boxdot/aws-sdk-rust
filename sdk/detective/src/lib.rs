#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! <p>Detective uses machine learning and purpose-built visualizations to help you analyze and
//! investigate security issues across your Amazon Web Services (AWS) workloads. Detective automatically
//! extracts time-based events such as login attempts, API calls, and network traffic from
//! AWS CloudTrail and Amazon Virtual Private Cloud (Amazon VPC) flow logs. It also extracts findings detected by
//! Amazon GuardDuty.</p>
//! <p>The Detective API primarily supports the creation and management of behavior graphs. A
//! behavior graph contains the extracted data from a set of member accounts, and is created
//! and managed by an administrator account.</p>
//! <p>Every behavior graph is specific to a Region. You can only use the API to manage graphs
//! that belong to the Region that is associated with the currently selected endpoint.</p>
//! <p>A Detective administrator account can use the Detective API to do the following:</p>
//! <ul>
//! <li>
//! <p>Enable and disable Detective. Enabling Detective creates a new behavior graph.</p>
//! </li>
//! <li>
//! <p>View the list of member accounts in a behavior graph.</p>
//! </li>
//! <li>
//! <p>Add member accounts to a behavior graph.</p>
//! </li>
//! <li>
//! <p>Remove member accounts from a behavior graph.</p>
//! </li>
//! </ul>
//! <p>A member account can use the Detective API to do the following:</p>
//! <ul>
//! <li>
//! <p>View the list of behavior graphs that they are invited to.</p>
//! </li>
//! <li>
//! <p>Accept an invitation to contribute to a behavior graph.</p>
//! </li>
//! <li>
//! <p>Decline an invitation to contribute to a behavior graph.</p>
//! </li>
//! <li>
//! <p>Remove their account from a behavior graph.</p>
//! </li>
//! </ul>
//! <p>All API actions are logged as CloudTrail events. See <a href="https://docs.aws.amazon.com/detective/latest/adminguide/logging-using-cloudtrail.html">Logging Detective API Calls with CloudTrail</a>.</p>
//! <note>
//! <p>We replaced the term "master account" with the term "administrator account." An
//! administrator account is used to centrally manage multiple accounts. In the case of
//! Detective, the administrator account manages the accounts in their behavior graph.</p>
//! </note>

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
mod json_deser;
mod json_errors;
mod json_ser;
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
    aws_http::user_agent::ApiMetadata::new("detective", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
