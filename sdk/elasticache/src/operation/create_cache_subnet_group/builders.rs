// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_cache_subnet_group::_create_cache_subnet_group_output::CreateCacheSubnetGroupOutputBuilder;

pub use crate::operation::create_cache_subnet_group::_create_cache_subnet_group_input::CreateCacheSubnetGroupInputBuilder;

/// Fluent builder constructing a request to `CreateCacheSubnetGroup`.
///
/// <p>Creates a new cache subnet group.</p>
/// <p>Use this parameter only when you are creating a cluster in an Amazon Virtual Private Cloud (Amazon VPC).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCacheSubnetGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::create_cache_subnet_group::builders::CreateCacheSubnetGroupInputBuilder,
}
impl CreateCacheSubnetGroupFluentBuilder {
    /// Creates a new `CreateCacheSubnetGroup`.
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
            crate::operation::create_cache_subnet_group::CreateCacheSubnetGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_cache_subnet_group::CreateCacheSubnetGroupError,
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
        crate::operation::create_cache_subnet_group::CreateCacheSubnetGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_cache_subnet_group::CreateCacheSubnetGroupError,
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
        crate::operation::create_cache_subnet_group::CreateCacheSubnetGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_cache_subnet_group::CreateCacheSubnetGroupError,
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
            crate::operation::create_cache_subnet_group::CreateCacheSubnetGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_cache_subnet_group::CreateCacheSubnetGroupError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A name for the cache subnet group. This value is stored as a lowercase string.</p>
    /// <p>Constraints: Must contain no more than 255 alphanumeric characters or hyphens.</p>
    /// <p>Example: <code>mysubnetgroup</code> </p>
    pub fn cache_subnet_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.cache_subnet_group_name(input.into());
        self
    }
    /// <p>A name for the cache subnet group. This value is stored as a lowercase string.</p>
    /// <p>Constraints: Must contain no more than 255 alphanumeric characters or hyphens.</p>
    /// <p>Example: <code>mysubnetgroup</code> </p>
    pub fn set_cache_subnet_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_cache_subnet_group_name(input);
        self
    }
    /// <p>A description for the cache subnet group.</p>
    pub fn cache_subnet_group_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.cache_subnet_group_description(input.into());
        self
    }
    /// <p>A description for the cache subnet group.</p>
    pub fn set_cache_subnet_group_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_cache_subnet_group_description(input);
        self
    }
    /// Appends an item to `SubnetIds`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>A list of VPC subnet IDs for the cache subnet group.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subnet_ids(input.into());
        self
    }
    /// <p>A list of VPC subnet IDs for the cache subnet group.</p>
    pub fn set_subnet_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_subnet_ids(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
