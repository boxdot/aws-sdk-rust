// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_launch_configuration_template::_update_launch_configuration_template_output::UpdateLaunchConfigurationTemplateOutputBuilder;

pub use crate::operation::update_launch_configuration_template::_update_launch_configuration_template_input::UpdateLaunchConfigurationTemplateInputBuilder;

/// Fluent builder constructing a request to `UpdateLaunchConfigurationTemplate`.
///
/// <p>Updates an existing Launch Configuration Template by ID.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateLaunchConfigurationTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_launch_configuration_template::builders::UpdateLaunchConfigurationTemplateInputBuilder,
}
impl UpdateLaunchConfigurationTemplateFluentBuilder {
    /// Creates a new `UpdateLaunchConfigurationTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplate, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplate, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateError>
    >{
        self.customize_middleware().await
    }
    /// <p>Launch Configuration Template ID.</p>
    pub fn launch_configuration_template_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.launch_configuration_template_id(input.into());
        self
    }
    /// <p>Launch Configuration Template ID.</p>
    pub fn set_launch_configuration_template_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_launch_configuration_template_id(input);
        self
    }
    /// <p>Launch disposition.</p>
    pub fn launch_disposition(mut self, input: crate::types::LaunchDisposition) -> Self {
        self.inner = self.inner.launch_disposition(input);
        self
    }
    /// <p>Launch disposition.</p>
    pub fn set_launch_disposition(
        mut self,
        input: ::std::option::Option<crate::types::LaunchDisposition>,
    ) -> Self {
        self.inner = self.inner.set_launch_disposition(input);
        self
    }
    /// <p>Target instance type right-sizing method.</p>
    pub fn target_instance_type_right_sizing_method(
        mut self,
        input: crate::types::TargetInstanceTypeRightSizingMethod,
    ) -> Self {
        self.inner = self.inner.target_instance_type_right_sizing_method(input);
        self
    }
    /// <p>Target instance type right-sizing method.</p>
    pub fn set_target_instance_type_right_sizing_method(
        mut self,
        input: ::std::option::Option<crate::types::TargetInstanceTypeRightSizingMethod>,
    ) -> Self {
        self.inner = self
            .inner
            .set_target_instance_type_right_sizing_method(input);
        self
    }
    /// <p>Copy private IP.</p>
    pub fn copy_private_ip(mut self, input: bool) -> Self {
        self.inner = self.inner.copy_private_ip(input);
        self
    }
    /// <p>Copy private IP.</p>
    pub fn set_copy_private_ip(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_copy_private_ip(input);
        self
    }
    /// <p>Copy tags.</p>
    pub fn copy_tags(mut self, input: bool) -> Self {
        self.inner = self.inner.copy_tags(input);
        self
    }
    /// <p>Copy tags.</p>
    pub fn set_copy_tags(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_copy_tags(input);
        self
    }
    /// <p>Licensing.</p>
    pub fn licensing(mut self, input: crate::types::Licensing) -> Self {
        self.inner = self.inner.licensing(input);
        self
    }
    /// <p>Licensing.</p>
    pub fn set_licensing(mut self, input: ::std::option::Option<crate::types::Licensing>) -> Self {
        self.inner = self.inner.set_licensing(input);
        self
    }
}
