// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_elastic_ips::_describe_elastic_ips_output::DescribeElasticIpsOutputBuilder;

pub use crate::operation::describe_elastic_ips::_describe_elastic_ips_input::DescribeElasticIpsInputBuilder;

/// Fluent builder constructing a request to `DescribeElasticIps`.
///
/// <p>Describes <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html">Elastic IP addresses</a>.</p> <note>
/// <p>This call accepts only one resource-identifying parameter.</p>
/// </note>
/// <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeElasticIpsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_elastic_ips::builders::DescribeElasticIpsInputBuilder,
}
impl DescribeElasticIpsFluentBuilder {
    /// Creates a new `DescribeElasticIps`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_elastic_ips::DescribeElasticIps,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_elastic_ips::DescribeElasticIpsError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_elastic_ips::DescribeElasticIpsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_elastic_ips::DescribeElasticIpsError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_elastic_ips::DescribeElasticIpsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_elastic_ips::DescribeElasticIpsError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_elastic_ips::DescribeElasticIps,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_elastic_ips::DescribeElasticIpsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The instance ID. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the Elastic IP addresses associated with the specified instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The instance ID. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the Elastic IP addresses associated with the specified instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>A stack ID. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the Elastic IP addresses that are registered with the specified stack.</p>
    pub fn stack_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stack_id(input.into());
        self
    }
    /// <p>A stack ID. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the Elastic IP addresses that are registered with the specified stack.</p>
    pub fn set_stack_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stack_id(input);
        self
    }
    /// Appends an item to `Ips`.
    ///
    /// To override the contents of this collection use [`set_ips`](Self::set_ips).
    ///
    /// <p>An array of Elastic IP addresses to be described. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the specified Elastic IP addresses. Otherwise, it returns a description of every Elastic IP address.</p>
    pub fn ips(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ips(input.into());
        self
    }
    /// <p>An array of Elastic IP addresses to be described. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the specified Elastic IP addresses. Otherwise, it returns a description of every Elastic IP address.</p>
    pub fn set_ips(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_ips(input);
        self
    }
}
