// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_firewall_policy::_describe_firewall_policy_output::DescribeFirewallPolicyOutputBuilder;

pub use crate::operation::describe_firewall_policy::_describe_firewall_policy_input::DescribeFirewallPolicyInputBuilder;

/// Fluent builder constructing a request to `DescribeFirewallPolicy`.
///
/// <p>Returns the data objects for the specified firewall policy. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeFirewallPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_firewall_policy::builders::DescribeFirewallPolicyInputBuilder,
}
impl DescribeFirewallPolicyFluentBuilder {
    /// Creates a new `DescribeFirewallPolicy`.
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
            crate::operation::describe_firewall_policy::DescribeFirewallPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_firewall_policy::DescribeFirewallPolicyError,
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
        crate::operation::describe_firewall_policy::DescribeFirewallPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_firewall_policy::DescribeFirewallPolicyError,
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
        crate::operation::describe_firewall_policy::DescribeFirewallPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_firewall_policy::DescribeFirewallPolicyError,
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
            crate::operation::describe_firewall_policy::DescribeFirewallPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_firewall_policy::DescribeFirewallPolicyError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The descriptive name of the firewall policy. You can't change the name of a firewall policy after you create it.</p>
    /// <p>You must specify the ARN or the name, and you can specify both. </p>
    pub fn firewall_policy_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.firewall_policy_name(input.into());
        self
    }
    /// <p>The descriptive name of the firewall policy. You can't change the name of a firewall policy after you create it.</p>
    /// <p>You must specify the ARN or the name, and you can specify both. </p>
    pub fn set_firewall_policy_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_firewall_policy_name(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the firewall policy.</p>
    /// <p>You must specify the ARN or the name, and you can specify both. </p>
    pub fn firewall_policy_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.firewall_policy_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the firewall policy.</p>
    /// <p>You must specify the ARN or the name, and you can specify both. </p>
    pub fn set_firewall_policy_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_firewall_policy_arn(input);
        self
    }
}
