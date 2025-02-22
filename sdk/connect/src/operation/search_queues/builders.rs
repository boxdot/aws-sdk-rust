// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_queues::_search_queues_output::SearchQueuesOutputBuilder;

pub use crate::operation::search_queues::_search_queues_input::SearchQueuesInputBuilder;

/// Fluent builder constructing a request to `SearchQueues`.
///
/// <p>This API is in preview release for Amazon Connect and is subject to change.</p>
/// <p>Searches queues in an Amazon Connect instance, with optional filtering.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SearchQueuesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_queues::builders::SearchQueuesInputBuilder,
}
impl SearchQueuesFluentBuilder {
    /// Creates a new `SearchQueues`.
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
            crate::operation::search_queues::SearchQueues,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::search_queues::SearchQueuesError>,
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
        crate::operation::search_queues::SearchQueuesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::search_queues::SearchQueuesError>,
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
        crate::operation::search_queues::SearchQueuesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::search_queues::SearchQueuesError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::search_queues::SearchQueues,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::search_queues::SearchQueuesError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::search_queues::paginator::SearchQueuesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::search_queues::paginator::SearchQueuesPaginator {
        crate::operation::search_queues::paginator::SearchQueuesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Filters to be applied to search results.</p>
    pub fn search_filter(mut self, input: crate::types::QueueSearchFilter) -> Self {
        self.inner = self.inner.search_filter(input);
        self
    }
    /// <p>Filters to be applied to search results.</p>
    pub fn set_search_filter(
        mut self,
        input: ::std::option::Option<crate::types::QueueSearchFilter>,
    ) -> Self {
        self.inner = self.inner.set_search_filter(input);
        self
    }
    /// <p>The search criteria to be used to return queues.</p> <note>
    /// <p>The <code>name</code> and <code>description</code> fields support "contains" queries with a minimum of 2 characters and a maximum of 25 characters. Any queries with character lengths outside of this range will throw invalid results. </p>
    /// </note>
    pub fn search_criteria(mut self, input: crate::types::QueueSearchCriteria) -> Self {
        self.inner = self.inner.search_criteria(input);
        self
    }
    /// <p>The search criteria to be used to return queues.</p> <note>
    /// <p>The <code>name</code> and <code>description</code> fields support "contains" queries with a minimum of 2 characters and a maximum of 25 characters. Any queries with character lengths outside of this range will throw invalid results. </p>
    /// </note>
    pub fn set_search_criteria(
        mut self,
        input: ::std::option::Option<crate::types::QueueSearchCriteria>,
    ) -> Self {
        self.inner = self.inner.set_search_criteria(input);
        self
    }
}
