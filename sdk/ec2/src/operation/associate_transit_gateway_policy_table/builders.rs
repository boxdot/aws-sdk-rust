// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_transit_gateway_policy_table::_associate_transit_gateway_policy_table_output::AssociateTransitGatewayPolicyTableOutputBuilder;

pub use crate::operation::associate_transit_gateway_policy_table::_associate_transit_gateway_policy_table_input::AssociateTransitGatewayPolicyTableInputBuilder;

/// Fluent builder constructing a request to `AssociateTransitGatewayPolicyTable`.
///
/// <p>Associates the specified transit gateway attachment with a transit gateway policy table.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateTransitGatewayPolicyTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::associate_transit_gateway_policy_table::builders::AssociateTransitGatewayPolicyTableInputBuilder,
}
impl AssociateTransitGatewayPolicyTableFluentBuilder {
    /// Creates a new `AssociateTransitGatewayPolicyTable`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::associate_transit_gateway_policy_table::AssociateTransitGatewayPolicyTable, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::associate_transit_gateway_policy_table::AssociateTransitGatewayPolicyTableError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::associate_transit_gateway_policy_table::AssociateTransitGatewayPolicyTableOutput, ::aws_smithy_http::result::SdkError<crate::operation::associate_transit_gateway_policy_table::AssociateTransitGatewayPolicyTableError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::associate_transit_gateway_policy_table::AssociateTransitGatewayPolicyTableOutput, ::aws_smithy_http::result::SdkError<crate::operation::associate_transit_gateway_policy_table::AssociateTransitGatewayPolicyTableError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::associate_transit_gateway_policy_table::AssociateTransitGatewayPolicyTable, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::associate_transit_gateway_policy_table::AssociateTransitGatewayPolicyTableError>
    >{
        self.customize_middleware().await
    }
    /// <p>The ID of the transit gateway policy table to associate with the transit gateway attachment.</p>
    pub fn transit_gateway_policy_table_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.transit_gateway_policy_table_id(input.into());
        self
    }
    /// <p>The ID of the transit gateway policy table to associate with the transit gateway attachment.</p>
    pub fn set_transit_gateway_policy_table_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_policy_table_id(input);
        self
    }
    /// <p>The ID of the transit gateway attachment to associate with the policy table.</p>
    pub fn transit_gateway_attachment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.transit_gateway_attachment_id(input.into());
        self
    }
    /// <p>The ID of the transit gateway attachment to associate with the policy table.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_attachment_id(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}
