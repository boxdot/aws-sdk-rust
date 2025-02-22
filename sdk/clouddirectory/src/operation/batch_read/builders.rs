// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_read::_batch_read_output::BatchReadOutputBuilder;

pub use crate::operation::batch_read::_batch_read_input::BatchReadInputBuilder;

/// Fluent builder constructing a request to `BatchRead`.
///
/// <p>Performs all the read operations in a batch. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchReadFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_read::builders::BatchReadInputBuilder,
}
impl BatchReadFluentBuilder {
    /// Creates a new `BatchRead`.
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
            crate::operation::batch_read::BatchRead,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::batch_read::BatchReadError>,
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
        crate::operation::batch_read::BatchReadOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::batch_read::BatchReadError>,
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
        crate::operation::batch_read::BatchReadOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::batch_read::BatchReadError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::batch_read::BatchRead,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::batch_read::BatchReadError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code>. For more information, see <code>arns</code>.</p>
    pub fn directory_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.directory_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code>. For more information, see <code>arns</code>.</p>
    pub fn set_directory_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_directory_arn(input);
        self
    }
    /// Appends an item to `Operations`.
    ///
    /// To override the contents of this collection use [`set_operations`](Self::set_operations).
    ///
    /// <p>A list of operations that are part of the batch.</p>
    pub fn operations(mut self, input: crate::types::BatchReadOperation) -> Self {
        self.inner = self.inner.operations(input);
        self
    }
    /// <p>A list of operations that are part of the batch.</p>
    pub fn set_operations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::BatchReadOperation>>,
    ) -> Self {
        self.inner = self.inner.set_operations(input);
        self
    }
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    pub fn consistency_level(mut self, input: crate::types::ConsistencyLevel) -> Self {
        self.inner = self.inner.consistency_level(input);
        self
    }
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    pub fn set_consistency_level(
        mut self,
        input: ::std::option::Option<crate::types::ConsistencyLevel>,
    ) -> Self {
        self.inner = self.inner.set_consistency_level(input);
        self
    }
}
