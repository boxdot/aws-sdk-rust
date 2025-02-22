// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_base_path_mapping::_get_base_path_mapping_output::GetBasePathMappingOutputBuilder;

pub use crate::operation::get_base_path_mapping::_get_base_path_mapping_input::GetBasePathMappingInputBuilder;

/// Fluent builder constructing a request to `GetBasePathMapping`.
///
/// <p>Describe a BasePathMapping resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetBasePathMappingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_base_path_mapping::builders::GetBasePathMappingInputBuilder,
}
impl GetBasePathMappingFluentBuilder {
    /// Creates a new `GetBasePathMapping`.
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
            crate::operation::get_base_path_mapping::GetBasePathMapping,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_base_path_mapping::GetBasePathMappingError,
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
        crate::operation::get_base_path_mapping::GetBasePathMappingOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_base_path_mapping::GetBasePathMappingError,
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
        crate::operation::get_base_path_mapping::GetBasePathMappingOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_base_path_mapping::GetBasePathMappingError,
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
            crate::operation::get_base_path_mapping::GetBasePathMapping,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_base_path_mapping::GetBasePathMappingError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The domain name of the BasePathMapping resource to be described.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The domain name of the BasePathMapping resource to be described.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The base path name that callers of the API must provide as part of the URL after the domain name. This value must be unique for all of the mappings across a single API. Specify '(none)' if you do not want callers to specify any base path name after the domain name.</p>
    pub fn base_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.base_path(input.into());
        self
    }
    /// <p>The base path name that callers of the API must provide as part of the URL after the domain name. This value must be unique for all of the mappings across a single API. Specify '(none)' if you do not want callers to specify any base path name after the domain name.</p>
    pub fn set_base_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_base_path(input);
        self
    }
}
