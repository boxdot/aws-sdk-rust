// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_replace_permission_associations_work::_list_replace_permission_associations_work_output::ListReplacePermissionAssociationsWorkOutputBuilder;

pub use crate::operation::list_replace_permission_associations_work::_list_replace_permission_associations_work_input::ListReplacePermissionAssociationsWorkInputBuilder;

/// Fluent builder constructing a request to `ListReplacePermissionAssociationsWork`.
///
/// <p>Retrieves the current status of the asynchronous tasks performed by RAM when you perform the <code>ReplacePermissionAssociationsWork</code> operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListReplacePermissionAssociationsWorkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::list_replace_permission_associations_work::builders::ListReplacePermissionAssociationsWorkInputBuilder,
}
impl ListReplacePermissionAssociationsWorkFluentBuilder {
    /// Creates a new `ListReplacePermissionAssociationsWork`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::list_replace_permission_associations_work::ListReplacePermissionAssociationsWork, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::list_replace_permission_associations_work::ListReplacePermissionAssociationsWorkError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::list_replace_permission_associations_work::ListReplacePermissionAssociationsWorkOutput, ::aws_smithy_http::result::SdkError<crate::operation::list_replace_permission_associations_work::ListReplacePermissionAssociationsWorkError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::list_replace_permission_associations_work::ListReplacePermissionAssociationsWorkOutput, ::aws_smithy_http::result::SdkError<crate::operation::list_replace_permission_associations_work::ListReplacePermissionAssociationsWorkError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::list_replace_permission_associations_work::ListReplacePermissionAssociationsWork, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::list_replace_permission_associations_work::ListReplacePermissionAssociationsWorkError>
    >{
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_replace_permission_associations_work::paginator::ListReplacePermissionAssociationsWorkPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_replace_permission_associations_work::paginator::ListReplacePermissionAssociationsWorkPaginator{
        crate::operation::list_replace_permission_associations_work::paginator::ListReplacePermissionAssociationsWorkPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `workIds`.
    ///
    /// To override the contents of this collection use [`set_work_ids`](Self::set_work_ids).
    ///
    /// <p>A list of IDs. These values come from the <code>id</code>field of the <code>replacePermissionAssociationsWork</code>structure returned by the <code>ReplacePermissionAssociations</code> operation. </p>
    pub fn work_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.work_ids(input.into());
        self
    }
    /// <p>A list of IDs. These values come from the <code>id</code>field of the <code>replacePermissionAssociationsWork</code>structure returned by the <code>ReplacePermissionAssociations</code> operation. </p>
    pub fn set_work_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_work_ids(input);
        self
    }
    /// <p>Specifies that you want to see only the details about requests with a status that matches this value.</p>
    pub fn status(mut self, input: crate::types::ReplacePermissionAssociationsWorkStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>Specifies that you want to see only the details about requests with a status that matches this value.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::ReplacePermissionAssociationsWorkStatus>,
    ) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>NextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>NextToken</code> response to request the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>NextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>NextToken</code> response to request the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Specifies the total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the number you specify, the <code>NextToken</code> response element is returned with a value (not null). Include the specified value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Specifies the total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the number you specify, the <code>NextToken</code> response element is returned with a value (not null). Include the specified value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
