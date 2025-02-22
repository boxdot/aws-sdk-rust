// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_managed_scaling_policy::_put_managed_scaling_policy_output::PutManagedScalingPolicyOutputBuilder;

pub use crate::operation::put_managed_scaling_policy::_put_managed_scaling_policy_input::PutManagedScalingPolicyInputBuilder;

/// Fluent builder constructing a request to `PutManagedScalingPolicy`.
///
/// <p>Creates or updates a managed scaling policy for an Amazon EMR cluster. The managed scaling policy defines the limits for resources, such as Amazon EC2 instances that can be added or terminated from a cluster. The policy only applies to the core and task nodes. The master node cannot be scaled after initial configuration. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutManagedScalingPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::put_managed_scaling_policy::builders::PutManagedScalingPolicyInputBuilder,
}
impl PutManagedScalingPolicyFluentBuilder {
    /// Creates a new `PutManagedScalingPolicy`.
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
            crate::operation::put_managed_scaling_policy::PutManagedScalingPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_managed_scaling_policy::PutManagedScalingPolicyError,
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
        crate::operation::put_managed_scaling_policy::PutManagedScalingPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_managed_scaling_policy::PutManagedScalingPolicyError,
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
        crate::operation::put_managed_scaling_policy::PutManagedScalingPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_managed_scaling_policy::PutManagedScalingPolicyError,
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
            crate::operation::put_managed_scaling_policy::PutManagedScalingPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_managed_scaling_policy::PutManagedScalingPolicyError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Specifies the ID of an Amazon EMR cluster where the managed scaling policy is attached. </p>
    pub fn cluster_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_id(input.into());
        self
    }
    /// <p>Specifies the ID of an Amazon EMR cluster where the managed scaling policy is attached. </p>
    pub fn set_cluster_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_id(input);
        self
    }
    /// <p>Specifies the constraints for the managed scaling policy. </p>
    pub fn managed_scaling_policy(mut self, input: crate::types::ManagedScalingPolicy) -> Self {
        self.inner = self.inner.managed_scaling_policy(input);
        self
    }
    /// <p>Specifies the constraints for the managed scaling policy. </p>
    pub fn set_managed_scaling_policy(
        mut self,
        input: ::std::option::Option<crate::types::ManagedScalingPolicy>,
    ) -> Self {
        self.inner = self.inner.set_managed_scaling_policy(input);
        self
    }
}
