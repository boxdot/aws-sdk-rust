// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_packaging_group::_create_packaging_group_output::CreatePackagingGroupOutputBuilder;

pub use crate::operation::create_packaging_group::_create_packaging_group_input::CreatePackagingGroupInputBuilder;

/// Fluent builder constructing a request to `CreatePackagingGroup`.
///
/// Creates a new MediaPackage VOD PackagingGroup resource.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePackagingGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_packaging_group::builders::CreatePackagingGroupInputBuilder,
}
impl CreatePackagingGroupFluentBuilder {
    /// Creates a new `CreatePackagingGroup`.
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
            crate::operation::create_packaging_group::CreatePackagingGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_packaging_group::CreatePackagingGroupError,
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
        crate::operation::create_packaging_group::CreatePackagingGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_packaging_group::CreatePackagingGroupError,
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
        crate::operation::create_packaging_group::CreatePackagingGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_packaging_group::CreatePackagingGroupError,
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
            crate::operation::create_packaging_group::CreatePackagingGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_packaging_group::CreatePackagingGroupError,
        >,
    > {
        self.customize_middleware().await
    }
    /// CDN Authorization credentials
    pub fn authorization(mut self, input: crate::types::Authorization) -> Self {
        self.inner = self.inner.authorization(input);
        self
    }
    /// CDN Authorization credentials
    pub fn set_authorization(
        mut self,
        input: ::std::option::Option<crate::types::Authorization>,
    ) -> Self {
        self.inner = self.inner.set_authorization(input);
        self
    }
    /// Configure egress access logging.
    pub fn egress_access_logs(mut self, input: crate::types::EgressAccessLogs) -> Self {
        self.inner = self.inner.egress_access_logs(input);
        self
    }
    /// Configure egress access logging.
    pub fn set_egress_access_logs(
        mut self,
        input: ::std::option::Option<crate::types::EgressAccessLogs>,
    ) -> Self {
        self.inner = self.inner.set_egress_access_logs(input);
        self
    }
    /// The ID of the PackagingGroup.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// The ID of the PackagingGroup.
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// A collection of tags associated with a resource
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// A collection of tags associated with a resource
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
