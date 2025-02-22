// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_device::_update_device_output::UpdateDeviceOutputBuilder;

pub use crate::operation::update_device::_update_device_input::UpdateDeviceInputBuilder;

/// Fluent builder constructing a request to `UpdateDevice`.
///
/// <p>Updates the details for an existing device. To remove information for any of the parameters, specify an empty string.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDeviceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_device::builders::UpdateDeviceInputBuilder,
}
impl UpdateDeviceFluentBuilder {
    /// Creates a new `UpdateDevice`.
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
            crate::operation::update_device::UpdateDevice,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_device::UpdateDeviceError>,
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
        crate::operation::update_device::UpdateDeviceOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_device::UpdateDeviceError>,
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
        crate::operation::update_device::UpdateDeviceOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_device::UpdateDeviceError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_device::UpdateDevice,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_device::UpdateDeviceError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.global_network_id(input.into());
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn set_global_network_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_network_id(input);
        self
    }
    /// <p>The ID of the device.</p>
    pub fn device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_id(input.into());
        self
    }
    /// <p>The ID of the device.</p>
    pub fn set_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_id(input);
        self
    }
    /// <p>The Amazon Web Services location of the device, if applicable. For an on-premises device, you can omit this parameter.</p>
    pub fn aws_location(mut self, input: crate::types::AwsLocation) -> Self {
        self.inner = self.inner.aws_location(input);
        self
    }
    /// <p>The Amazon Web Services location of the device, if applicable. For an on-premises device, you can omit this parameter.</p>
    pub fn set_aws_location(
        mut self,
        input: ::std::option::Option<crate::types::AwsLocation>,
    ) -> Self {
        self.inner = self.inner.set_aws_location(input);
        self
    }
    /// <p>A description of the device.</p>
    /// <p>Constraints: Maximum length of 256 characters.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the device.</p>
    /// <p>Constraints: Maximum length of 256 characters.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The type of the device.</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.r#type(input.into());
        self
    }
    /// <p>The type of the device.</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The vendor of the device.</p>
    /// <p>Constraints: Maximum length of 128 characters.</p>
    pub fn vendor(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vendor(input.into());
        self
    }
    /// <p>The vendor of the device.</p>
    /// <p>Constraints: Maximum length of 128 characters.</p>
    pub fn set_vendor(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vendor(input);
        self
    }
    /// <p>The model of the device.</p>
    /// <p>Constraints: Maximum length of 128 characters.</p>
    pub fn model(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model(input.into());
        self
    }
    /// <p>The model of the device.</p>
    /// <p>Constraints: Maximum length of 128 characters.</p>
    pub fn set_model(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model(input);
        self
    }
    /// <p>The serial number of the device.</p>
    /// <p>Constraints: Maximum length of 128 characters.</p>
    pub fn serial_number(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.serial_number(input.into());
        self
    }
    /// <p>The serial number of the device.</p>
    /// <p>Constraints: Maximum length of 128 characters.</p>
    pub fn set_serial_number(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_serial_number(input);
        self
    }
    /// <p>Describes a location.</p>
    pub fn location(mut self, input: crate::types::Location) -> Self {
        self.inner = self.inner.location(input);
        self
    }
    /// <p>Describes a location.</p>
    pub fn set_location(mut self, input: ::std::option::Option<crate::types::Location>) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
    /// <p>The ID of the site.</p>
    pub fn site_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.site_id(input.into());
        self
    }
    /// <p>The ID of the site.</p>
    pub fn set_site_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_site_id(input);
        self
    }
}
