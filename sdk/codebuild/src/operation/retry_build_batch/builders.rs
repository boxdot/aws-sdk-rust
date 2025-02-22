// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::retry_build_batch::_retry_build_batch_output::RetryBuildBatchOutputBuilder;

pub use crate::operation::retry_build_batch::_retry_build_batch_input::RetryBuildBatchInputBuilder;

/// Fluent builder constructing a request to `RetryBuildBatch`.
///
/// <p>Restarts a failed batch build. Only batch builds that have failed can be retried.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RetryBuildBatchFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::retry_build_batch::builders::RetryBuildBatchInputBuilder,
}
impl RetryBuildBatchFluentBuilder {
    /// Creates a new `RetryBuildBatch`.
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
            crate::operation::retry_build_batch::RetryBuildBatch,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::retry_build_batch::RetryBuildBatchError,
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
        crate::operation::retry_build_batch::RetryBuildBatchOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::retry_build_batch::RetryBuildBatchError,
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
        crate::operation::retry_build_batch::RetryBuildBatchOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::retry_build_batch::RetryBuildBatchError,
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
            crate::operation::retry_build_batch::RetryBuildBatch,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::retry_build_batch::RetryBuildBatchError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Specifies the identifier of the batch build to restart.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>Specifies the identifier of the batch build to restart.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>A unique, case sensitive identifier you provide to ensure the idempotency of the <code>RetryBuildBatch</code> request. The token is included in the <code>RetryBuildBatch</code> request and is valid for five minutes. If you repeat the <code>RetryBuildBatch</code> request with the same token, but change a parameter, CodeBuild returns a parameter mismatch error.</p>
    pub fn idempotency_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.idempotency_token(input.into());
        self
    }
    /// <p>A unique, case sensitive identifier you provide to ensure the idempotency of the <code>RetryBuildBatch</code> request. The token is included in the <code>RetryBuildBatch</code> request and is valid for five minutes. If you repeat the <code>RetryBuildBatch</code> request with the same token, but change a parameter, CodeBuild returns a parameter mismatch error.</p>
    pub fn set_idempotency_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_idempotency_token(input);
        self
    }
    /// <p>Specifies the type of retry to perform.</p>
    pub fn retry_type(mut self, input: crate::types::RetryBuildBatchType) -> Self {
        self.inner = self.inner.retry_type(input);
        self
    }
    /// <p>Specifies the type of retry to perform.</p>
    pub fn set_retry_type(
        mut self,
        input: ::std::option::Option<crate::types::RetryBuildBatchType>,
    ) -> Self {
        self.inner = self.inner.set_retry_type(input);
        self
    }
}
