// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_configuration_set_sending_enabled::_update_configuration_set_sending_enabled_output::UpdateConfigurationSetSendingEnabledOutputBuilder;

pub use crate::operation::update_configuration_set_sending_enabled::_update_configuration_set_sending_enabled_input::UpdateConfigurationSetSendingEnabledInputBuilder;

/// Fluent builder constructing a request to `UpdateConfigurationSetSendingEnabled`.
///
/// <p>Enables or disables email sending for messages sent using a specific configuration set in a given AWS Region. You can use this operation in conjunction with Amazon CloudWatch alarms to temporarily pause email sending for a configuration set when the reputation metrics for that configuration set (such as your bounce on complaint rate) exceed certain thresholds.</p>
/// <p>You can execute this operation no more than once per second.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateConfigurationSetSendingEnabledFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_configuration_set_sending_enabled::builders::UpdateConfigurationSetSendingEnabledInputBuilder,
}
impl UpdateConfigurationSetSendingEnabledFluentBuilder {
    /// Creates a new `UpdateConfigurationSetSendingEnabled`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::update_configuration_set_sending_enabled::UpdateConfigurationSetSendingEnabled, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::update_configuration_set_sending_enabled::UpdateConfigurationSetSendingEnabledError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::update_configuration_set_sending_enabled::UpdateConfigurationSetSendingEnabledOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_configuration_set_sending_enabled::UpdateConfigurationSetSendingEnabledError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::update_configuration_set_sending_enabled::UpdateConfigurationSetSendingEnabledOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_configuration_set_sending_enabled::UpdateConfigurationSetSendingEnabledError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::update_configuration_set_sending_enabled::UpdateConfigurationSetSendingEnabled, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::update_configuration_set_sending_enabled::UpdateConfigurationSetSendingEnabledError>
    >{
        self.customize_middleware().await
    }
    /// <p>The name of the configuration set that you want to update.</p>
    pub fn configuration_set_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.configuration_set_name(input.into());
        self
    }
    /// <p>The name of the configuration set that you want to update.</p>
    pub fn set_configuration_set_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_set_name(input);
        self
    }
    /// <p>Describes whether email sending is enabled or disabled for the configuration set. </p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.enabled(input);
        self
    }
    /// <p>Describes whether email sending is enabled or disabled for the configuration set. </p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enabled(input);
        self
    }
}
