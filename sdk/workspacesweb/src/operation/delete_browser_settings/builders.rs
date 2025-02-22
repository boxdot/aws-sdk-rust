// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_browser_settings::_delete_browser_settings_output::DeleteBrowserSettingsOutputBuilder;

pub use crate::operation::delete_browser_settings::_delete_browser_settings_input::DeleteBrowserSettingsInputBuilder;

/// Fluent builder constructing a request to `DeleteBrowserSettings`.
///
/// <p>Deletes browser settings.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteBrowserSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_browser_settings::builders::DeleteBrowserSettingsInputBuilder,
}
impl DeleteBrowserSettingsFluentBuilder {
    /// Creates a new `DeleteBrowserSettings`.
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
            crate::operation::delete_browser_settings::DeleteBrowserSettings,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_browser_settings::DeleteBrowserSettingsError,
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
        crate::operation::delete_browser_settings::DeleteBrowserSettingsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_browser_settings::DeleteBrowserSettingsError,
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
        crate::operation::delete_browser_settings::DeleteBrowserSettingsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_browser_settings::DeleteBrowserSettingsError,
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
            crate::operation::delete_browser_settings::DeleteBrowserSettings,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_browser_settings::DeleteBrowserSettingsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ARN of the browser settings.</p>
    pub fn browser_settings_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.browser_settings_arn(input.into());
        self
    }
    /// <p>The ARN of the browser settings.</p>
    pub fn set_browser_settings_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_browser_settings_arn(input);
        self
    }
}
