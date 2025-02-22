// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_disassociate_client_device_from_core_device::_batch_disassociate_client_device_from_core_device_output::BatchDisassociateClientDeviceFromCoreDeviceOutputBuilder;

pub use crate::operation::batch_disassociate_client_device_from_core_device::_batch_disassociate_client_device_from_core_device_input::BatchDisassociateClientDeviceFromCoreDeviceInputBuilder;

/// Fluent builder constructing a request to `BatchDisassociateClientDeviceFromCoreDevice`.
///
/// <p>Disassociates a list of client devices from a core device. After you disassociate a client device from a core device, the client device won't be able to use cloud discovery to retrieve the core device's connectivity information and certificates.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchDisassociateClientDeviceFromCoreDeviceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::batch_disassociate_client_device_from_core_device::builders::BatchDisassociateClientDeviceFromCoreDeviceInputBuilder,
}
impl BatchDisassociateClientDeviceFromCoreDeviceFluentBuilder {
    /// Creates a new `BatchDisassociateClientDeviceFromCoreDevice`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDevice, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceOutput, ::aws_smithy_http::result::SdkError<crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceOutput, ::aws_smithy_http::result::SdkError<crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDevice, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceError>
    >{
        self.customize_middleware().await
    }
    /// Appends an item to `entries`.
    ///
    /// To override the contents of this collection use [`set_entries`](Self::set_entries).
    ///
    /// <p>The list of client devices to disassociate.</p>
    pub fn entries(
        mut self,
        input: crate::types::DisassociateClientDeviceFromCoreDeviceEntry,
    ) -> Self {
        self.inner = self.inner.entries(input);
        self
    }
    /// <p>The list of client devices to disassociate.</p>
    pub fn set_entries(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::DisassociateClientDeviceFromCoreDeviceEntry>,
        >,
    ) -> Self {
        self.inner = self.inner.set_entries(input);
        self
    }
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    pub fn core_device_thing_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.core_device_thing_name(input.into());
        self
    }
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    pub fn set_core_device_thing_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_core_device_thing_name(input);
        self
    }
}
