// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_resource_evaluation::_start_resource_evaluation_output::StartResourceEvaluationOutputBuilder;

pub use crate::operation::start_resource_evaluation::_start_resource_evaluation_input::StartResourceEvaluationInputBuilder;

/// Fluent builder constructing a request to `StartResourceEvaluation`.
///
/// <p>Runs an on-demand evaluation for the specified resource to determine whether the resource details will comply with configured Config rules. You can also use it for evaluation purposes. Config recommends using an evaluation context. It runs an execution against the resource details with all of the Config rules in your account that match with the specified proactive mode and resource type.</p> <note>
/// <p>Ensure you have the <code>cloudformation:DescribeType</code> role setup to validate the resource type schema.</p>
/// <p>You can find the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html">Resource type schema</a> in "<i>Amazon Web Services public extensions</i>" within the CloudFormation registry or with the following CLI commmand: <code>aws cloudformation describe-type --type-name "AWS::S3::Bucket" --type RESOURCE</code>.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/registry.html#registry-view">Managing extensions through the CloudFormation registry</a> and <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Amazon Web Services resource and property types reference</a> in the CloudFormation User Guide.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartResourceEvaluationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::start_resource_evaluation::builders::StartResourceEvaluationInputBuilder,
}
impl StartResourceEvaluationFluentBuilder {
    /// Creates a new `StartResourceEvaluation`.
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
            crate::operation::start_resource_evaluation::StartResourceEvaluation,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_resource_evaluation::StartResourceEvaluationError,
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
        crate::operation::start_resource_evaluation::StartResourceEvaluationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_resource_evaluation::StartResourceEvaluationError,
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
        crate::operation::start_resource_evaluation::StartResourceEvaluationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_resource_evaluation::StartResourceEvaluationError,
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
            crate::operation::start_resource_evaluation::StartResourceEvaluation,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_resource_evaluation::StartResourceEvaluationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Returns a <code>ResourceDetails</code> object.</p>
    pub fn resource_details(mut self, input: crate::types::ResourceDetails) -> Self {
        self.inner = self.inner.resource_details(input);
        self
    }
    /// <p>Returns a <code>ResourceDetails</code> object.</p>
    pub fn set_resource_details(
        mut self,
        input: ::std::option::Option<crate::types::ResourceDetails>,
    ) -> Self {
        self.inner = self.inner.set_resource_details(input);
        self
    }
    /// <p>Returns an <code>EvaluationContext</code> object.</p>
    pub fn evaluation_context(mut self, input: crate::types::EvaluationContext) -> Self {
        self.inner = self.inner.evaluation_context(input);
        self
    }
    /// <p>Returns an <code>EvaluationContext</code> object.</p>
    pub fn set_evaluation_context(
        mut self,
        input: ::std::option::Option<crate::types::EvaluationContext>,
    ) -> Self {
        self.inner = self.inner.set_evaluation_context(input);
        self
    }
    /// <p>The mode of an evaluation. The valid values for this API are <code>DETECTIVE</code> and <code>PROACTIVE</code>.</p>
    pub fn evaluation_mode(mut self, input: crate::types::EvaluationMode) -> Self {
        self.inner = self.inner.evaluation_mode(input);
        self
    }
    /// <p>The mode of an evaluation. The valid values for this API are <code>DETECTIVE</code> and <code>PROACTIVE</code>.</p>
    pub fn set_evaluation_mode(
        mut self,
        input: ::std::option::Option<crate::types::EvaluationMode>,
    ) -> Self {
        self.inner = self.inner.set_evaluation_mode(input);
        self
    }
    /// <p>The timeout for an evaluation. The default is 900 seconds. You cannot specify a number greater than 3600. If you specify 0, Config uses the default.</p>
    pub fn evaluation_timeout(mut self, input: i32) -> Self {
        self.inner = self.inner.evaluation_timeout(input);
        self
    }
    /// <p>The timeout for an evaluation. The default is 900 seconds. You cannot specify a number greater than 3600. If you specify 0, Config uses the default.</p>
    pub fn set_evaluation_timeout(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_evaluation_timeout(input);
        self
    }
    /// <p>A client token is a unique, case-sensitive string of up to 64 ASCII characters. To make an idempotent API request using one of these actions, specify a client token in the request.</p> <note>
    /// <p>Avoid reusing the same client token for other API requests. If you retry a request that completed successfully using the same client token and the same parameters, the retry succeeds without performing any further actions. If you retry a successful request using the same client token, but one or more of the parameters are different, other than the Region or Availability Zone, the retry fails with an IdempotentParameterMismatch error.</p>
    /// </note>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A client token is a unique, case-sensitive string of up to 64 ASCII characters. To make an idempotent API request using one of these actions, specify a client token in the request.</p> <note>
    /// <p>Avoid reusing the same client token for other API requests. If you retry a request that completed successfully using the same client token and the same parameters, the retry succeeds without performing any further actions. If you retry a successful request using the same client token, but one or more of the parameters are different, other than the Region or Availability Zone, the retry fails with an IdempotentParameterMismatch error.</p>
    /// </note>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
