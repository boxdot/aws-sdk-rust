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
//! <fullname>Amazon Relational Database Service</fullname>
//! <p></p>
//!
//! <p>Amazon Relational Database Service (Amazon RDS) is a web service that makes it easier to set up, operate, and
//! scale a relational database in the cloud. It provides cost-efficient, resizeable capacity for an industry-standard relational
//! database and manages common database administration tasks, freeing up developers to focus on what makes their applications
//! and businesses unique.</p>
//! <p>Amazon RDS gives you access to the capabilities of a MySQL, MariaDB, PostgreSQL, Microsoft SQL Server,
//! Oracle, or Amazon Aurora database server. These capabilities mean that the code, applications, and tools
//! you already use today with your existing databases work with Amazon RDS without modification. Amazon RDS
//! automatically backs up your database and maintains the database software that powers your DB instance. Amazon RDS
//! is flexible: you can scale your DB instance's compute resources and storage capacity to meet your
//! application's demand. As with all Amazon Web Services, there are no up-front investments, and you pay only for
//! the resources you use.</p>
//! <p>This interface reference for Amazon RDS contains documentation for a programming or command line interface
//! you can use to manage Amazon RDS. Amazon RDS is asynchronous, which means that some interfaces might
//! require techniques such as polling or callback functions to determine when a command has been applied. In this
//! reference, the parameter descriptions indicate whether a command is applied immediately, on the next instance reboot,
//! or during the maintenance window. The reference structure is as follows, and we list following some related topics
//! from the user guide.</p>
//! <p>
//! <b>Amazon RDS API Reference</b>
//! </p>
//! <ul>
//! <li>
//! <p>For the alphabetical list of API actions, see
//! <a href="https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_Operations.html">API Actions</a>.</p>
//! </li>
//! <li>
//! <p>For the alphabetical list of data types, see
//! <a href="https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_Types.html">Data Types</a>.</p>
//! </li>
//! <li>
//! <p>For a list of common query parameters, see
//! <a href="https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/CommonParameters.html">Common Parameters</a>.</p>
//! </li>
//! <li>
//! <p>For descriptions of the error codes, see
//! <a href="https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/CommonErrors.html">Common Errors</a>.</p>
//! </li>
//! </ul>
//! <p>
//! <b>Amazon RDS User Guide</b>
//! </p>
//! <ul>
//! <li>
//! <p>For a summary of the Amazon RDS interfaces, see
//! <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Welcome.html#Welcome.Interfaces">Available RDS Interfaces</a>.</p>
//! </li>
//! <li>
//! <p>For more information about how to use the Query API, see
//! <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Using_the_Query_API.html">Using the Query API</a>.</p>
//! </li>
//! </ul>
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
//!
//! # Examples
//! Examples can be found [here](https://github.com/awslabs/aws-sdk-rust/tree/main/examples/rds).

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
    aws_http::user_agent::ApiMetadata::new("rds", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
