// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_audit_suppression::_create_audit_suppression_output::CreateAuditSuppressionOutputBuilder;

pub use crate::operation::create_audit_suppression::_create_audit_suppression_input::CreateAuditSuppressionInputBuilder;

/// Fluent builder constructing a request to `CreateAuditSuppression`.
///
/// <p> Creates a Device Defender audit suppression. </p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">CreateAuditSuppression</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAuditSuppressionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_audit_suppression::builders::CreateAuditSuppressionInputBuilder,
}
impl CreateAuditSuppressionFluentBuilder {
    /// Creates a new `CreateAuditSuppression`.
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
            crate::operation::create_audit_suppression::CreateAuditSuppression,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_audit_suppression::CreateAuditSuppressionError,
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
        crate::operation::create_audit_suppression::CreateAuditSuppressionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_audit_suppression::CreateAuditSuppressionError,
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
        crate::operation::create_audit_suppression::CreateAuditSuppressionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_audit_suppression::CreateAuditSuppressionError,
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
            crate::operation::create_audit_suppression::CreateAuditSuppression,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_audit_suppression::CreateAuditSuppressionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>An audit check name. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    pub fn check_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.check_name(input.into());
        self
    }
    /// <p>An audit check name. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    pub fn set_check_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_check_name(input);
        self
    }
    /// <p>Information that identifies the noncompliant resource.</p>
    pub fn resource_identifier(mut self, input: crate::types::ResourceIdentifier) -> Self {
        self.inner = self.inner.resource_identifier(input);
        self
    }
    /// <p>Information that identifies the noncompliant resource.</p>
    pub fn set_resource_identifier(
        mut self,
        input: ::std::option::Option<crate::types::ResourceIdentifier>,
    ) -> Self {
        self.inner = self.inner.set_resource_identifier(input);
        self
    }
    /// <p> The epoch timestamp in seconds at which this suppression expires. </p>
    pub fn expiration_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.expiration_date(input);
        self
    }
    /// <p> The epoch timestamp in seconds at which this suppression expires. </p>
    pub fn set_expiration_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_expiration_date(input);
        self
    }
    /// <p> Indicates whether a suppression should exist indefinitely or not. </p>
    pub fn suppress_indefinitely(mut self, input: bool) -> Self {
        self.inner = self.inner.suppress_indefinitely(input);
        self
    }
    /// <p> Indicates whether a suppression should exist indefinitely or not. </p>
    pub fn set_suppress_indefinitely(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_suppress_indefinitely(input);
        self
    }
    /// <p> The description of the audit suppression. </p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p> The description of the audit suppression. </p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p> Each audit supression must have a unique client request token. If you try to create a new audit suppression with the same token as one that already exists, an exception occurs. If you omit this value, Amazon Web Services SDKs will automatically generate a unique client request.</p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p> Each audit supression must have a unique client request token. If you try to create a new audit suppression with the same token as one that already exists, an exception occurs. If you omit this value, Amazon Web Services SDKs will automatically generate a unique client request.</p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
}
