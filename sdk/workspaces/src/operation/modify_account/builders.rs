// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_account::_modify_account_output::ModifyAccountOutputBuilder;

pub use crate::operation::modify_account::_modify_account_input::ModifyAccountInputBuilder;

/// Fluent builder constructing a request to `ModifyAccount`.
///
/// <p>Modifies the configuration of Bring Your Own License (BYOL) for the specified account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyAccountFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_account::builders::ModifyAccountInputBuilder,
}
impl ModifyAccountFluentBuilder {
    /// Creates a new `ModifyAccount`.
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
            crate::operation::modify_account::ModifyAccount,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_account::ModifyAccountError>,
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
        crate::operation::modify_account::ModifyAccountOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_account::ModifyAccountError>,
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
        crate::operation::modify_account::ModifyAccountOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_account::ModifyAccountError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::modify_account::ModifyAccount,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_account::ModifyAccountError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The status of BYOL.</p>
    pub fn dedicated_tenancy_support(
        mut self,
        input: crate::types::DedicatedTenancySupportEnum,
    ) -> Self {
        self.inner = self.inner.dedicated_tenancy_support(input);
        self
    }
    /// <p>The status of BYOL.</p>
    pub fn set_dedicated_tenancy_support(
        mut self,
        input: ::std::option::Option<crate::types::DedicatedTenancySupportEnum>,
    ) -> Self {
        self.inner = self.inner.set_dedicated_tenancy_support(input);
        self
    }
    /// <p>The IP address range, specified as an IPv4 CIDR block, for the management network interface. Specify an IP address range that is compatible with your network and in CIDR notation (that is, specify the range as an IPv4 CIDR block). The CIDR block size must be /16 (for example, 203.0.113.25/16). It must also be specified as available by the <code>ListAvailableManagementCidrRanges</code> operation.</p>
    pub fn dedicated_tenancy_management_cidr_range(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self
            .inner
            .dedicated_tenancy_management_cidr_range(input.into());
        self
    }
    /// <p>The IP address range, specified as an IPv4 CIDR block, for the management network interface. Specify an IP address range that is compatible with your network and in CIDR notation (that is, specify the range as an IPv4 CIDR block). The CIDR block size must be /16 (for example, 203.0.113.25/16). It must also be specified as available by the <code>ListAvailableManagementCidrRanges</code> operation.</p>
    pub fn set_dedicated_tenancy_management_cidr_range(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self
            .inner
            .set_dedicated_tenancy_management_cidr_range(input);
        self
    }
}
