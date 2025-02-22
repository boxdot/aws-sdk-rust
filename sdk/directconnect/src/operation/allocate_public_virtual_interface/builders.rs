// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::allocate_public_virtual_interface::_allocate_public_virtual_interface_output::AllocatePublicVirtualInterfaceOutputBuilder;

pub use crate::operation::allocate_public_virtual_interface::_allocate_public_virtual_interface_input::AllocatePublicVirtualInterfaceInputBuilder;

/// Fluent builder constructing a request to `AllocatePublicVirtualInterface`.
///
/// <p>Provisions a public virtual interface to be owned by the specified Amazon Web Services account.</p>
/// <p>The owner of a connection calls this function to provision a public virtual interface to be owned by the specified Amazon Web Services account.</p>
/// <p>Virtual interfaces created using this function must be confirmed by the owner using <code>ConfirmPublicVirtualInterface</code>. Until this step has been completed, the virtual interface is in the <code>confirming</code> state and is not available to handle traffic.</p>
/// <p>When creating an IPv6 public virtual interface, omit the Amazon address and customer address. IPv6 addresses are automatically assigned from the Amazon pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AllocatePublicVirtualInterfaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::allocate_public_virtual_interface::builders::AllocatePublicVirtualInterfaceInputBuilder,
}
impl AllocatePublicVirtualInterfaceFluentBuilder {
    /// Creates a new `AllocatePublicVirtualInterface`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterface, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceOutput, ::aws_smithy_http::result::SdkError<crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceOutput, ::aws_smithy_http::result::SdkError<crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterface, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::allocate_public_virtual_interface::AllocatePublicVirtualInterfaceError>
    >{
        self.customize_middleware().await
    }
    /// <p>The ID of the connection on which the public virtual interface is provisioned.</p>
    pub fn connection_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.connection_id(input.into());
        self
    }
    /// <p>The ID of the connection on which the public virtual interface is provisioned.</p>
    pub fn set_connection_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_connection_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the public virtual interface.</p>
    pub fn owner_account(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.owner_account(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the public virtual interface.</p>
    pub fn set_owner_account(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_owner_account(input);
        self
    }
    /// <p>Information about the public virtual interface.</p>
    pub fn new_public_virtual_interface_allocation(
        mut self,
        input: crate::types::NewPublicVirtualInterfaceAllocation,
    ) -> Self {
        self.inner = self.inner.new_public_virtual_interface_allocation(input);
        self
    }
    /// <p>Information about the public virtual interface.</p>
    pub fn set_new_public_virtual_interface_allocation(
        mut self,
        input: ::std::option::Option<crate::types::NewPublicVirtualInterfaceAllocation>,
    ) -> Self {
        self.inner = self
            .inner
            .set_new_public_virtual_interface_allocation(input);
        self
    }
}
