// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_resolver_query_log_config::_disassociate_resolver_query_log_config_output::DisassociateResolverQueryLogConfigOutputBuilder;

pub use crate::operation::disassociate_resolver_query_log_config::_disassociate_resolver_query_log_config_input::DisassociateResolverQueryLogConfigInputBuilder;

/// Fluent builder constructing a request to `DisassociateResolverQueryLogConfig`.
///
/// <p>Disassociates a VPC from a query logging configuration.</p> <note>
/// <p>Before you can delete a query logging configuration, you must first disassociate all VPCs from the configuration. If you used Resource Access Manager (RAM) to share a query logging configuration with other accounts, VPCs can be disassociated from the configuration in the following ways:</p>
/// <ul>
/// <li> <p>The accounts that you shared the configuration with can disassociate VPCs from the configuration.</p> </li>
/// <li> <p>You can stop sharing the configuration.</p> </li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateResolverQueryLogConfigFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::disassociate_resolver_query_log_config::builders::DisassociateResolverQueryLogConfigInputBuilder,
}
impl DisassociateResolverQueryLogConfigFluentBuilder {
    /// Creates a new `DisassociateResolverQueryLogConfig`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::disassociate_resolver_query_log_config::DisassociateResolverQueryLogConfig, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::disassociate_resolver_query_log_config::DisassociateResolverQueryLogConfigError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::disassociate_resolver_query_log_config::DisassociateResolverQueryLogConfigOutput, ::aws_smithy_http::result::SdkError<crate::operation::disassociate_resolver_query_log_config::DisassociateResolverQueryLogConfigError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::disassociate_resolver_query_log_config::DisassociateResolverQueryLogConfigOutput, ::aws_smithy_http::result::SdkError<crate::operation::disassociate_resolver_query_log_config::DisassociateResolverQueryLogConfigError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::disassociate_resolver_query_log_config::DisassociateResolverQueryLogConfig, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::disassociate_resolver_query_log_config::DisassociateResolverQueryLogConfigError>
    >{
        self.customize_middleware().await
    }
    /// <p>The ID of the query logging configuration that you want to disassociate a specified VPC from.</p>
    pub fn resolver_query_log_config_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.resolver_query_log_config_id(input.into());
        self
    }
    /// <p>The ID of the query logging configuration that you want to disassociate a specified VPC from.</p>
    pub fn set_resolver_query_log_config_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_resolver_query_log_config_id(input);
        self
    }
    /// <p>The ID of the Amazon VPC that you want to disassociate from a specified query logging configuration.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The ID of the Amazon VPC that you want to disassociate from a specified query logging configuration.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
}
