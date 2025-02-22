// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_workspaces::_list_workspaces_output::ListWorkspacesOutputBuilder;

pub use crate::operation::list_workspaces::_list_workspaces_input::ListWorkspacesInputBuilder;

/// Fluent builder constructing a request to `ListWorkspaces`.
///
/// Lists all AMP workspaces, including workspaces being created or deleted.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListWorkspacesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_workspaces::builders::ListWorkspacesInputBuilder,
}
impl ListWorkspacesFluentBuilder {
    /// Creates a new `ListWorkspaces`.
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
            crate::operation::list_workspaces::ListWorkspaces,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_workspaces::ListWorkspacesError>,
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
        crate::operation::list_workspaces::ListWorkspacesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_workspaces::ListWorkspacesError>,
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
        crate::operation::list_workspaces::ListWorkspacesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_workspaces::ListWorkspacesError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_workspaces::ListWorkspaces,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_workspaces::ListWorkspacesError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_workspaces::paginator::ListWorkspacesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_workspaces::paginator::ListWorkspacesPaginator {
        crate::operation::list_workspaces::paginator::ListWorkspacesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Pagination token to request the next page in a paginated list. This token is obtained from the output of the previous ListWorkspaces request.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// Pagination token to request the next page in a paginated list. This token is obtained from the output of the previous ListWorkspaces request.
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// Optional filter for workspace alias. Only the workspaces with aliases that begin with this value will be returned.
    pub fn alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alias(input.into());
        self
    }
    /// Optional filter for workspace alias. Only the workspaces with aliases that begin with this value will be returned.
    pub fn set_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alias(input);
        self
    }
    /// Maximum results to return in response (default=100, maximum=1000).
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// Maximum results to return in response (default=100, maximum=1000).
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
