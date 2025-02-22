// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_image_build_versions::_list_image_build_versions_output::ListImageBuildVersionsOutputBuilder;

pub use crate::operation::list_image_build_versions::_list_image_build_versions_input::ListImageBuildVersionsInputBuilder;

/// Fluent builder constructing a request to `ListImageBuildVersions`.
///
/// <p>Returns a list of image build versions.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListImageBuildVersionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::list_image_build_versions::builders::ListImageBuildVersionsInputBuilder,
}
impl ListImageBuildVersionsFluentBuilder {
    /// Creates a new `ListImageBuildVersions`.
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
            crate::operation::list_image_build_versions::ListImageBuildVersions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_image_build_versions::ListImageBuildVersionsError,
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
        crate::operation::list_image_build_versions::ListImageBuildVersionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_image_build_versions::ListImageBuildVersionsError,
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
        crate::operation::list_image_build_versions::ListImageBuildVersionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_image_build_versions::ListImageBuildVersionsError,
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
            crate::operation::list_image_build_versions::ListImageBuildVersions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_image_build_versions::ListImageBuildVersionsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_image_build_versions::paginator::ListImageBuildVersionsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_image_build_versions::paginator::ListImageBuildVersionsPaginator
    {
        crate::operation::list_image_build_versions::paginator::ListImageBuildVersionsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The Amazon Resource Name (ARN) of the image whose build versions you want to retrieve.</p>
    pub fn image_version_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.image_version_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the image whose build versions you want to retrieve.</p>
    pub fn set_image_version_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_image_version_arn(input);
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Use the following filters to streamline results:</p>
    /// <ul>
    /// <li> <p> <code>name</code> </p> </li>
    /// <li> <p> <code>osVersion</code> </p> </li>
    /// <li> <p> <code>platform</code> </p> </li>
    /// <li> <p> <code>type</code> </p> </li>
    /// <li> <p> <code>version</code> </p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Use the following filters to streamline results:</p>
    /// <ul>
    /// <li> <p> <code>name</code> </p> </li>
    /// <li> <p> <code>osVersion</code> </p> </li>
    /// <li> <p> <code>platform</code> </p> </li>
    /// <li> <p> <code>type</code> </p> </li>
    /// <li> <p> <code>version</code> </p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The maximum items to return in a request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum items to return in a request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
