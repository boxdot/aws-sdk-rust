// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_source_server_action::_remove_source_server_action_output::RemoveSourceServerActionOutputBuilder;

pub use crate::operation::remove_source_server_action::_remove_source_server_action_input::RemoveSourceServerActionInputBuilder;

/// Fluent builder constructing a request to `RemoveSourceServerAction`.
///
/// <p>Remove source server post migration custom action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemoveSourceServerActionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::remove_source_server_action::builders::RemoveSourceServerActionInputBuilder,
}
impl RemoveSourceServerActionFluentBuilder {
    /// Creates a new `RemoveSourceServerAction`.
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
            crate::operation::remove_source_server_action::RemoveSourceServerAction,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_source_server_action::RemoveSourceServerActionError,
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
        crate::operation::remove_source_server_action::RemoveSourceServerActionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_source_server_action::RemoveSourceServerActionError,
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
        crate::operation::remove_source_server_action::RemoveSourceServerActionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_source_server_action::RemoveSourceServerActionError,
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
            crate::operation::remove_source_server_action::RemoveSourceServerAction,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_source_server_action::RemoveSourceServerActionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Source server ID of the post migration custom action to remove.</p>
    pub fn source_server_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.source_server_id(input.into());
        self
    }
    /// <p>Source server ID of the post migration custom action to remove.</p>
    pub fn set_source_server_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_server_id(input);
        self
    }
    /// <p>Source server post migration custom action ID to remove.</p>
    pub fn action_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.action_id(input.into());
        self
    }
    /// <p>Source server post migration custom action ID to remove.</p>
    pub fn set_action_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_action_id(input);
        self
    }
}
