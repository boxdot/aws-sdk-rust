// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::abort_multipart_read_set_upload::_abort_multipart_read_set_upload_output::AbortMultipartReadSetUploadOutputBuilder;

pub use crate::operation::abort_multipart_read_set_upload::_abort_multipart_read_set_upload_input::AbortMultipartReadSetUploadInputBuilder;

/// Fluent builder constructing a request to `AbortMultipartReadSetUpload`.
///
/// <p> Stops a multipart upload. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AbortMultipartReadSetUploadFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::abort_multipart_read_set_upload::builders::AbortMultipartReadSetUploadInputBuilder,
}
impl AbortMultipartReadSetUploadFluentBuilder {
    /// Creates a new `AbortMultipartReadSetUpload`.
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
            crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUpload,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadError,
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
        crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadError,
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
        crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadError,
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
            crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUpload,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p> The sequence store ID for the store involved in the multipart upload. </p>
    pub fn sequence_store_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.sequence_store_id(input.into());
        self
    }
    /// <p> The sequence store ID for the store involved in the multipart upload. </p>
    pub fn set_sequence_store_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_sequence_store_id(input);
        self
    }
    /// <p> The ID for the multipart upload. </p>
    pub fn upload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.upload_id(input.into());
        self
    }
    /// <p> The ID for the multipart upload. </p>
    pub fn set_upload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_upload_id(input);
        self
    }
}
