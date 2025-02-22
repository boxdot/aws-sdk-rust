// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_ml_transforms::_list_ml_transforms_output::ListMlTransformsOutputBuilder;

pub use crate::operation::list_ml_transforms::_list_ml_transforms_input::ListMlTransformsInputBuilder;

/// Fluent builder constructing a request to `ListMLTransforms`.
///
/// <p> Retrieves a sortable, filterable list of existing Glue machine learning transforms in this Amazon Web Services account, or the resources with the specified tag. This operation takes the optional <code>Tags</code> field, which you can use as a filter of the responses so that tagged resources can be retrieved as a group. If you choose to use tag filtering, only resources with the tags are retrieved. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListMLTransformsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_ml_transforms::builders::ListMlTransformsInputBuilder,
}
impl ListMLTransformsFluentBuilder {
    /// Creates a new `ListMLTransforms`.
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
            crate::operation::list_ml_transforms::ListMLTransforms,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_ml_transforms::ListMLTransformsError,
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
        crate::operation::list_ml_transforms::ListMlTransformsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_ml_transforms::ListMLTransformsError,
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
        crate::operation::list_ml_transforms::ListMlTransformsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_ml_transforms::ListMLTransformsError,
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
            crate::operation::list_ml_transforms::ListMLTransforms,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_ml_transforms::ListMLTransformsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_ml_transforms::paginator::ListMlTransformsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_ml_transforms::paginator::ListMlTransformsPaginator {
        crate::operation::list_ml_transforms::paginator::ListMlTransformsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>A continuation token, if this is a continuation request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A continuation token, if this is a continuation request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum size of a list to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum size of a list to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>A <code>TransformFilterCriteria</code> used to filter the machine learning transforms.</p>
    pub fn filter(mut self, input: crate::types::TransformFilterCriteria) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p>A <code>TransformFilterCriteria</code> used to filter the machine learning transforms.</p>
    pub fn set_filter(
        mut self,
        input: ::std::option::Option<crate::types::TransformFilterCriteria>,
    ) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// <p>A <code>TransformSortCriteria</code> used to sort the machine learning transforms.</p>
    pub fn sort(mut self, input: crate::types::TransformSortCriteria) -> Self {
        self.inner = self.inner.sort(input);
        self
    }
    /// <p>A <code>TransformSortCriteria</code> used to sort the machine learning transforms.</p>
    pub fn set_sort(
        mut self,
        input: ::std::option::Option<crate::types::TransformSortCriteria>,
    ) -> Self {
        self.inner = self.inner.set_sort(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Specifies to return only these tagged resources.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Specifies to return only these tagged resources.</p>
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
