// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_domain_controllers::_describe_domain_controllers_output::DescribeDomainControllersOutputBuilder;

pub use crate::operation::describe_domain_controllers::_describe_domain_controllers_input::DescribeDomainControllersInputBuilder;

/// Fluent builder constructing a request to `DescribeDomainControllers`.
///
/// <p>Provides information about any domain controllers in your directory.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeDomainControllersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_domain_controllers::builders::DescribeDomainControllersInputBuilder,
}
impl DescribeDomainControllersFluentBuilder {
    /// Creates a new `DescribeDomainControllers`.
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
            crate::operation::describe_domain_controllers::DescribeDomainControllers,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_domain_controllers::DescribeDomainControllersError,
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
        crate::operation::describe_domain_controllers::DescribeDomainControllersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_domain_controllers::DescribeDomainControllersError,
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
        crate::operation::describe_domain_controllers::DescribeDomainControllersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_domain_controllers::DescribeDomainControllersError,
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
            crate::operation::describe_domain_controllers::DescribeDomainControllers,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_domain_controllers::DescribeDomainControllersError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_domain_controllers::paginator::DescribeDomainControllersPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_domain_controllers::paginator::DescribeDomainControllersPaginator
    {
        crate::operation::describe_domain_controllers::paginator::DescribeDomainControllersPaginator::new(self.handle, self.inner)
    }
    /// <p>Identifier of the directory for which to retrieve the domain controller information.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>Identifier of the directory for which to retrieve the domain controller information.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// Appends an item to `DomainControllerIds`.
    ///
    /// To override the contents of this collection use [`set_domain_controller_ids`](Self::set_domain_controller_ids).
    ///
    /// <p>A list of identifiers for the domain controllers whose information will be provided.</p>
    pub fn domain_controller_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.domain_controller_ids(input.into());
        self
    }
    /// <p>A list of identifiers for the domain controllers whose information will be provided.</p>
    pub fn set_domain_controller_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_domain_controller_ids(input);
        self
    }
    /// <p>The <i>DescribeDomainControllers.NextToken</i> value from a previous call to <code>DescribeDomainControllers</code>. Pass null if this is the first call. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <i>DescribeDomainControllers.NextToken</i> value from a previous call to <code>DescribeDomainControllers</code>. Pass null if this is the first call. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of items to return.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of items to return.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
}
